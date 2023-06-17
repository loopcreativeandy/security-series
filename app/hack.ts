import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor"
import { SecuritySeries, IDL} from "../target/types/security_series"
const kpFile = "../And2SoZPuWgG1QtunZ5LvCxwVwzBtkaTH1bBU2eLE2tA.json"
const fs = require("fs")
const kp : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile).toString())),
);
const wallet = new Wallet(kp);
const c = new web3.Connection("http://localhost:8899")
const provider = new AnchorProvider(c, wallet, {});
const programId = new web3.PublicKey("SECmF7dX572jE1S6KGchN6uxi9TMXwPZWUwArfQdgYn")
const program = new Program<SecuritySeries>(IDL, programId, provider)

async function run() {
    // const [pda, bump] = web3.PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("claimed", "utf8"),
    //     kp.publicKey.toBuffer()
    //   ],
    //   programId
    // );
    const bump = 253;
    const pda = web3.PublicKey.createProgramAddressSync(
      [
        Buffer.from("claimed", "utf8"),
        kp.publicKey.toBuffer(),
        Uint8Array.from([bump])
      ],
      programId
    );
    console.log("using bump "+bump)
    const sx = await program.methods.distribute(bump)
    .accounts({
        user: kp.publicKey,
        // claimed: pda,
        systemProgram: web3.SystemProgram.programId
    })
    .rpc();
    console.log(sx)
}
run();