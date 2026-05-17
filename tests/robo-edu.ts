import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RoboEdu } from "../target/types/robo_edu";
import { assert } from "chai";

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


  it("Creates a vehicle", async () => {
    
    // Step 1: define our inputs
    const vehicleId = new anchor.BN(1);
    const totalShares = new anchor.BN(1000);

    // Step 2: derive the PDA address (same seeds as Rust)
    const [vehiclePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vehicle"), vehicleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Step 3: call create_vehicle
    await program.methods
      .createVehicle(vehicleId, totalShares)
      .accounts({
        authority: provider.wallet.publicKey,
        vehicleAccount: vehiclePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Step 4: fetch the account from on-chain
    const vehicle = await program.account.vehicleAccount.fetch(vehiclePda);

    // Step 5: assert every field
    assert.equal(vehicle.vehicleId.toString(), "1");
    assert.equal(vehicle.totalShares.toString(), "1000");
    assert.equal(vehicle.sharesIssued.toString(), "0");
    assert.equal(vehicle.revenueAccumulated.toString(), "0");
    assert.equal(vehicle.isActive, false);
    assert.equal(
      vehicle.authority.toString(),
      provider.wallet.publicKey.toString()
    );

    console.log("✅ Vehicle created at:", vehiclePda.toString());
  });


});


