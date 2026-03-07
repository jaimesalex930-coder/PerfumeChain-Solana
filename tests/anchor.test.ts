// No imports needed: web3, anchor, pg and more are globally available

describe("Perfumeria", () => {

  it("Crear perfumeria", async () => {

    const [perfumeriaPDA] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("perfumeria"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    const txHash = await pg.program.methods
      .crearPerfumeria("Mi Perfumeria")
      .accounts({
        owner: pg.wallet.publicKey,
        perfumeria: perfumeriaPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", txHash);

  });


  it("Agregar perfume", async () => {

    const [perfumeriaPDA] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("perfumeria"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    const txHash = await pg.program.methods
      .agregarPerfume("Sauvage", "Dior", 2500)
      .accounts({
        owner: pg.wallet.publicKey,
        perfumeria: perfumeriaPDA,
      })
      .rpc();

    console.log("Perfume agregado:", txHash);

  });


  it("Ver perfumes", async () => {

    const [perfumeriaPDA] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("perfumeria"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    await pg.program.methods
      .verPerfumes()
      .accounts({
        owner: pg.wallet.publicKey,
        perfumeria: perfumeriaPDA,
      })
      .rpc();

    console.log("Perfumes consultados");

  });

});
