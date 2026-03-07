// Client

console.log("Wallet:", pg.wallet.publicKey.toString());

const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);


// Obtener PDA de la perfumeria
const [perfumeriaPDA] = web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("perfumeria"),
    pg.wallet.publicKey.toBuffer()
  ],
  pg.program.programId
);


// Crear perfumeria
await pg.program.methods
  .crearPerfumeria("Mi Perfumeria")
  .accounts({
    owner: pg.wallet.publicKey,
    perfumeria: perfumeriaPDA,
    systemProgram: web3.SystemProgram.programId
  })
  .rpc();

console.log("Perfumeria creada");


// Agregar perfume
await pg.program.methods
  .agregarPerfume("Sauvage", "Dior", 2500)
  .accounts({
    owner: pg.wallet.publicKey,
    perfumeria: perfumeriaPDA
  })
  .rpc();

console.log("Perfume agregado");


// Ver perfumes
await pg.program.methods
  .verPerfumes()
  .accounts({
    owner: pg.wallet.publicKey,
    perfumeria: perfumeriaPDA
  })
  .rpc();

console.log("Perfumes consultados");
