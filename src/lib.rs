use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod perfumeria {
    use super::*;

    //////////////////////////// Crear Perfumeria ////////////////////////////
    pub fn crear_perfumeria(context: Context<NuevaPerfumeria>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let perfumes: Vec<Perfume> = Vec::new();

        context.accounts.perfumeria.set_inner(Perfumeria {
            owner: owner_id,
            nombre,
            perfumes,
        });

        Ok(())
    }

    //////////////////////////// Agregar Perfume ////////////////////////////
    pub fn agregar_perfume(
        context: Context<NuevoPerfume>,
        nombre: String,
        marca: String,
        precio: u32
    ) -> Result<()> {

        require!(
            context.accounts.perfumeria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let perfume = Perfume {
            nombre,
            marca,
            precio,
            disponible: true,
        };

        context.accounts.perfumeria.perfumes.push(perfume);

        Ok(())
    }

    //////////////////////////// Eliminar Perfume ////////////////////////////
    pub fn eliminar_perfume(context: Context<NuevoPerfume>, nombre: String) -> Result<()> {

        require!(
            context.accounts.perfumeria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let perfumes = &mut context.accounts.perfumeria.perfumes;

        for i in 0..perfumes.len() {

            if perfumes[i].nombre == nombre {

                perfumes.remove(i);
                msg!("Perfume {} eliminado!", nombre);

                return Ok(());

            }

        }

        Err(Errores::PerfumeNoExiste.into())
    }

    //////////////////////////// Ver Perfumes ////////////////////////////
    pub fn ver_perfumes(context: Context<NuevoPerfume>) -> Result<()> {

        require!(
            context.accounts.perfumeria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista de perfumes: {:#?}",
            context.accounts.perfumeria.perfumes
        );

        Ok(())
    }

    //////////////////////////// Alternar Disponibilidad ////////////////////////////
    pub fn alternar_estado(context: Context<NuevoPerfume>, nombre: String) -> Result<()> {

        require!(
            context.accounts.perfumeria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let perfumes = &mut context.accounts.perfumeria.perfumes;

        for i in 0..perfumes.len() {

            let estado = perfumes[i].disponible;

            if perfumes[i].nombre == nombre {

                let nuevo_estado = !estado;

                perfumes[i].disponible = nuevo_estado;

                msg!(
                    "El perfume {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());

            }

        }

        Err(Errores::PerfumeNoExiste.into())
    }
}

//////////////////////////// Errores ////////////////////////////

#[error_code]
pub enum Errores {

    #[msg("Error, no eres el propietario de la perfumeria")]
    NoEresElOwner,

    #[msg("Error, el perfume no existe")]
    PerfumeNoExiste,

}

//////////////////////////// Cuenta Perfumeria ////////////////////////////

#[account]
#[derive(InitSpace)]

pub struct Perfumeria {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    perfumes: Vec<Perfume>,

}

//////////////////////////// Struct Perfume ////////////////////////////

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Debug
)]

pub struct Perfume {

    #[max_len(60)]
    nombre: String,

    #[max_len(40)]
    marca: String,

    precio: u32,

    disponible: bool,

}

//////////////////////////// Contexto Crear Perfumeria ////////////////////////////

#[derive(Accounts)]

pub struct NuevaPerfumeria<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Perfumeria::INIT_SPACE + 8,
        seeds = [b"perfumeria", owner.key().as_ref()],
        bump
    )]

    pub perfumeria: Account<'info, Perfumeria>,

    pub system_program: Program<'info, System>,

}

//////////////////////////// Contexto Perfumes ////////////////////////////

#[derive(Accounts)]

pub struct NuevoPerfume<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub perfumeria: Account<'info, Perfumeria>,

}
