import { Program, web3, workspace } from "@coral-xyz/anchor";
import { SmartContract } from "../target/types/smart_contract";

export const repo = {
  owner: "JulioMh",
  name: "github-rewards",
  branch: "main",
};

export const program = workspace.SmartContract as Program<SmartContract>;

export const [repoPda] = web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("repo"),
    Buffer.from(repo.owner),
    Buffer.from(repo.name),
    Buffer.from(repo.branch),
  ],
  program.programId
);

export const [mint] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("token")],
  program.programId
);
