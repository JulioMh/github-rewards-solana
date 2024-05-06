import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { program, repo, repoPda } from "./utils";

describe("vote_repo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const [votePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vote"), provider.publicKey.toBuffer(), repoPda.toBuffer()],
    program.programId
  );

  describe("happy path", () => {
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
  describe("errors", () => {
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
