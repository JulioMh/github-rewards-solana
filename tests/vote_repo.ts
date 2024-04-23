import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SmartContract } from "../target/types/smart_contract";
import { expect } from "chai";

describe("vote_repo", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SmartContract as Program<SmartContract>;

  const repo = {
    owner: "JulioMh",
    name: "github-rewards",
    branch: "main",
  };

  const [repoPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("repo"),
      Buffer.from(repo.owner),
      Buffer.from(repo.name),
      Buffer.from(repo.branch),
    ],
    program.programId
  );

  const [votePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vote"), provider.publicKey.toBuffer(), repoPda.toBuffer()],
    program.programId
  );

  describe("Initialize", () => {
    it("create repo", async () => {
      await program.methods
        .addRepo({ owner: repo.owner, name: repo.name, branch: repo.branch })
        .accounts({ publisher: provider.publicKey })
        .rpc();

      const account = await program.account.repo.fetch(repoPda);

      expect(account.publisher.toString()).equals(
        provider.publicKey.toString()
      );
      expect(account.votes.toNumber()).equals(0);
      expect(account.approvedTimestamp.toNumber()).equals(0);
      expect(account.owner).equals(repo.owner);
      expect(account.name).equals(repo.name);
      expect(account.branch).equals(repo.branch);
    });
  });

  describe("Happy path", () => {
    it("vote up", async () => {
      await program.methods
        .voteRepo({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          voteType: { up: {} },
        })
        .accounts({ voter: provider.publicKey })
        .rpc();

      const vote_res = await program.account.vote.fetch(votePda);
      const repo_res = await program.account.repo.fetch(repoPda);
      expect(vote_res.repoPda.toString()).equals(repoPda.toString());
      expect(repo_res.votes.toNumber()).equals(1);
    });

    it("vote down", async () => {
      const newUser = anchor.web3.Keypair.generate();
      await provider.connection.requestAirdrop(newUser.publicKey, 10000000);

      await program.methods
        .voteRepo({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          voteType: { down: {} },
        })
        .signers([newUser])
        .accounts({ voter: newUser.publicKey })
        .rpc({ skipPreflight: true });

      const [votePda1] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote"), newUser.publicKey.toBuffer(), repoPda.toBuffer()],
        program.programId
      );
      const vote_res = await program.account.vote.fetch(votePda1);
      const repo_res = await program.account.repo.fetch(repoPda);
      expect(vote_res.repoPda.toString()).equals(repoPda.toString());
      expect(repo_res.votes.toNumber()).equals(0);
    });

    it("change vote", async () => {
      await program.methods
        .voteRepo({
          repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
          voteType: { down: {} },
        })
        .accounts({ voter: provider.publicKey })
        .rpc();

      const vote_res = await program.account.vote.fetch(votePda);
      const repo_res = await program.account.repo.fetch(repoPda);
      expect(vote_res.repoPda.toString()).equals(repoPda.toString());
      expect(repo_res.votes.toNumber()).equals(-2);
    });
  });
  describe("Errors", () => {
    it("prevent voting the same twice", async () => {
      try {
        await program.methods
          .voteRepo({
            repo: { owner: repo.owner, name: repo.name, branch: repo.branch },
            voteType: { down: {} },
          })
          .accounts({ voter: provider.publicKey })
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.error.errorMessage).eq("User has already voted");
      }
    });
  });
});
