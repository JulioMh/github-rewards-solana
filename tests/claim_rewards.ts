import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SmartContract } from "../target/types/smart_contract";
import { repo, repoPda, mint } from "./utils";
import { expect } from "chai";

describe("claim_rewards", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SmartContract as Program<SmartContract>;

  const [rewardPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("reward"), provider.publicKey.toBuffer(), repoPda.toBuffer()],
    program.programId
  );

  const destination = anchor.utils.token.associatedAddress({
    mint,
    owner: provider.publicKey,
  });

  describe("happy path", () => {
    it("first time claiming rewards", async () => {
      const now = Date.now();
      await program.methods
        .claimRewards({
          repo,
          commits: new anchor.BN(10),
          timestamp: new anchor.BN(now),
        })
        .accounts({ destination })
        .rpc();

      const balance = await provider.connection.getTokenAccountBalance(
        destination
      );

      expect(balance.value.uiAmount).eq(10);

      const reward = await program.account.reward.fetch(rewardPda);
      expect(reward.totalClaimed.toNumber()).eq(10);
    });
  });
  describe("errors", () => {});
});
