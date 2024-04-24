import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { program, repo, repoPda } from "./utils";

describe("add_repo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("create repo", async () => {
    await program.methods
      .addRepo({ owner: repo.owner, name: repo.name, branch: repo.branch })
      .accounts({ publisher: provider.publicKey })
      .rpc();

    const account = await program.account.repo.fetch(repoPda);

    expect(account.publisher.toString()).equals(provider.publicKey.toString());
    expect(account.votes.toNumber()).equals(0);
    expect(account.approvedTimestamp.toNumber()).equals(0);
    expect(account.owner).equals(repo.owner);
    expect(account.name).equals(repo.name);
    expect(account.branch).equals(repo.branch);
  });
});
