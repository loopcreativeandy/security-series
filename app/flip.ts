import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor"
import { SecuritySeries, IDL} from "../target/types/security_series"
import { SYSVAR_SLOT_HASHES_PUBKEY,SYSVAR_INSTRUCTIONS_PUBKEY} from "@solana/web3.js"
const kpFile1 = "../keys/And2SoZPuWgG1QtunZ5LvCxwVwzBtkaTH1bBU2eLE2tA.json"
const fs = require("fs")
const kp1 : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile1).toString())),
);
const wallet = new Wallet(kp1);
const c = new web3.Connection("https://api.devnet.solana.com")
const provider = new AnchorProvider(c, wallet, {});
const programId = new web3.PublicKey("CaskxYs2fbFggrf1wsccAQGRKL3FgGM8vWUsJ1khMdHs")
const program = new Program<SecuritySeries>(IDL, programId, provider)

async function flip() {
  
  const ix1 = await program.methods.flip()
  .accounts({
      player: kp1.publicKey,
      sysvarSlothahsesAccount: SYSVAR_SLOT_HASHES_PUBKEY,
      sysvarInstructionsAccount: SYSVAR_INSTRUCTIONS_PUBKEY
  })
  .instruction();
  
  const tx = new web3.Transaction().add(ix1);

  const sx = await c.sendTransaction(tx, [kp1], {skipPreflight: true});
  console.log(sx)
  
}
flip();