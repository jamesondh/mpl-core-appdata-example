import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { MplCoreAppdataExample } from "../target/types/mpl_core_appdata_example";

describe("mpl-core-appdata-example", () => {
  /// Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.Wallet.local();
  const program = anchor.workspace
    .MplCoreAppdataExample as Program<MplCoreAppdataExample>;

  const coreProgram = new PublicKey(
    "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
  );

  const manager = PublicKey.createProgramAddressSync(
    [Buffer.from("manager")],
    program.programId
  )[0];

  it("Setup Manager", async () => {
    const tx = await program.methods
      .setupManager()
      .accountsPartial({
        signer: wallet.publicKey,
        payer: wallet.publicKey,
        manager,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();

    console.log(tx);
  });

  const createAssetArgs = {
    name: "Asset 1",
    uri: "https://example.com",
  };

  const asset = Keypair.generate();

  it("Create Ticket", async () => {
    const tx = await program.methods
      .createAsset(createAssetArgs)
      .accountsPartial({
        signer: wallet.publicKey,
        payer: wallet.publicKey,
        manager,
        asset: asset.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: coreProgram,
      })
      .signers([wallet.payer, asset])
      .rpc();

    console.log(tx);
  });
});
