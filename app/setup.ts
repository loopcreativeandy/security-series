import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor"
import { SecuritySeries, IDL} from "../target/types/security_series"
const kpFile2 = "../keys/2ndVbymu2MG5C95YU4bcb5KvM2PkWhVkdfkhuNhk78UH.json"
const kpFile1 = "../keys/And2SoZPuWgG1QtunZ5LvCxwVwzBtkaTH1bBU2eLE2tA.json"
const fs = require("fs")
const kp1 : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile1).toString())),
);
const kp2 : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile2).toString())),
);
const wallet = new Wallet(kp1);
const c = new web3.Connection("https://api.devnet.solana.com")
const provider = new AnchorProvider(c, wallet, {});
const programId = new web3.PublicKey("CaskxYs2fbFggrf1wsccAQGRKL3FgGM8vWUsJ1khMdHs")
const program = new Program<SecuritySeries>(IDL, programId, provider)

async function setup() {
  
  const ix1 = await program.methods.initTreasury()
  .accounts({
      payer: kp1.publicKey
  })
  .instruction();
  
  const tx = new web3.Transaction().add(ix1);

  const sx = await c.sendTransaction(tx, [kp1]);
  console.log(sx)
  
}
setup();