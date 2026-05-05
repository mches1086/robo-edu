import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RoboEdu } from "../target/types/robo_edu";

describe("robo-edu", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RoboEdu as Program<RoboEdu>;

  it("Initializes Robotaxi!", async () => {
    const tx = await program.methods
      .initialize()
      .rpc();

    console.log("✅ Transaction signature:", tx);
  });
});