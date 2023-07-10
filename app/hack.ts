import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor"
import { SecuritySeries, IDL} from "../target/types/security_series"
const kpFile1 = "../2ndVbymu2MG5C95YU4bcb5KvM2PkWhVkdfkhuNhk78UH.json"
const kpFile2 = "../And2SoZPuWgG1QtunZ5LvCxwVwzBtkaTH1bBU2eLE2tA.json"
const fs = require("fs")
const kp1 : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile1).toString())),
);
const kp2 : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile2).toString())),
);
const wallet = new Wallet(kp1);
const c = new web3.Connection("http://127.0.0.1:8899")
const provider = new AnchorProvider(c, wallet, {});
const programId = new web3.PublicKey("SECmF7dX572jE1S6KGchN6uxi9TMXwPZWUwArfQdgYn")
const program = new Program<SecuritySeries>(IDL, programId, provider)

const playerAccount = new web3.PublicKey("Cc5LpZPFw7ShHRTcsT12hzETLP4vYapmuCodiSjc9jP7");

async function play(round : number) {
  
  const ix1 = await program.methods.play(round)
  .accounts({
      player1: kp1.publicKey,
      player2: kp1.publicKey,
      sysvarSlothahsesAccount: web3.SYSVAR_SLOT_HASHES_PUBKEY
  })
  .signers([kp2])
  .instruction();
  const ix2 = await program.methods.evaluate()
  .accounts({
      player1: kp1.publicKey,
      player2: kp1.publicKey,
      sysvarSlothahsesAccount: web3.SYSVAR_SLOT_HASHES_PUBKEY
  })
  .instruction();

  const tx = new web3.Transaction().add(ix1, ix2);

  const sx = await c.sendTransaction(tx, [kp1]);
  console.log(sx)
  
}

play(1)
// for(let i = 1; i<7; i++){
//   play(i);
//   await new Promise(_ => setTimeout(_, 2000));
// }