import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fuzzme } from "../target/types/fuzzme";
import { BN } from "bn.js";

describe("fuzzme", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Fuzzme as Program<Fuzzme>;

  it("test with 0!", async () => {
    // Add your test here.
    const tx = await program.methods.fuzzme(new BN(0)).rpc();
    console.log("Your transaction signature", tx);
  });
  it("test with max value!", async () => {
    // Add your test here.
    const tx = await program.methods.fuzzme(new BN(-1)).rpc();
    console.log("Your transaction signature", tx);
  });
  it("test with 42!", async () => {
    // Add your test here.
    const tx = await program.methods.fuzzme(new BN(42)).rpc();
    console.log("Your transaction signature", tx);
  });
});
