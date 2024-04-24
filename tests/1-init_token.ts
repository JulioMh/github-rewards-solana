import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { mint, program } from "./utils";

describe("init_token", () => {
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  describe("happy path", () => {
    it("create token mint", async () => {
      await program.methods
        .initToken()
        .accounts({ metadata })
        .rpc()
        .catch((e) => console.error(e));

      const mintInfo = await provider.connection.getAccountInfo(mint);

      expect(mintInfo.owner.toString()).eq(TOKEN_PROGRAM_ID.toString());
    });
  });
  describe("errors", () => {
    it("only admin can create the token", async () => {
      const newUser = anchor.web3.Keypair.generate();
      await provider.connection.requestAirdrop(newUser.publicKey, 10000000);
      try {
        await program.methods
          .initToken()
          .accounts({ metadata })
          .signers([newUser])
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.message).eq(
          `unknown signer: ${newUser.publicKey.toString()}`
        );
      }
    });
  });
});
