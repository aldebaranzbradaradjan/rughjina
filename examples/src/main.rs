rughjina::rughjina! {
    esternu cassa rughjina;

    usu std::collections::Dizziunariu cumè Dizziu;

    trattu ChjaveVaglia {
        funzione scrive(&mè, chjave: Catena, vaglia: Catena);
        funzione leghje(&mè, chjave: Catena) -> Risultatu<PòEsse<&Catena>, Catena>;
    }

    staticu mutabile DIZZIUNARIU: PòEsse<Dizziu<Catena, Catena>> = Nunda;

    struttura Cuncreta;

    realizazione ChjaveVaglia per Cuncreta {
        funzione scrive(&mè, chjave: Catena, vaglia: Catena) {
            lascia dizziu = periculosu {
                DIZZIUNARIU.piglià_o_inseritu_cun(Difettu::difettu)
            };
            dizziu.inserì(chjave, vaglia);
        }
        funzione leghje(&mè, chjave: Catena) -> Risultatu<PòEsse<&Catena>, Catena> {
            si lascia UnPocu(dizziu) = periculosu { DIZZIUNARIU.cumè_rif() } {
                VaBè(dizziu.ritruvà(&chjave))
            } sinnò {
                Err("Cerca in u dizziunariu".comu())
            }
        }
    }

    publicu(cassa) funzione pò_esse(i: u32) -> PòEsse<Risultatu<u32, Catena>> {
        si i % 2 == 1 {
            si i == 42 {
                UnPocu(Err(Catena::da("Merde")))
            } sinnò {
                UnPocu(VaBè(33))
            }
        } sinnò {
            Nunda
        }
    }

    asincronu funzione esempiu() {
    }

    asincronu funzione esempiu2() {
        esempiu().aspetta;
    }

    funzione principale() {
        lascia mutabile x = 31;

        currisponde x {
            42 => {
                stampatuln!("Frittata à u Brocciu")
            }
            _ => stampatuln!("Eccu")
        }

        per i di 0..10 {
            lascia val = ricciuli {
                stutà i;
            };

            finchì x < val {
                x += 1;
            }

            x = si lascia UnPocu(risultatu) = pò_esse(i) {
                risultatu.sballà()
            } sinnò {
                12
            };
        }

        //secondaire();
    }

    #[cuncedente(codice_micca_accessibile)]
    funzione secundariu() {
        merda!("oh nò !"); // for the true Corsican experience
        cazzu!("buffonu"); // for diversity
        mancatu!("mi sò sbagliatu"); // in SFW contexts
    }
}
