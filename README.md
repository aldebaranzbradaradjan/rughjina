# rughjina

![](https://github.com/aldebaranzbradaradjan/rughjina/raw/main/logo.jpeg)

Aren't you _stancu_ from writing Rust programs in English? Do you like saying
"merda" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Corsican touch to your
programs?

**rughjina** (Corsican for _Rust_) is here to save your day, as it allows you to
write Rust programs in Corsican, using Corsican keywords, Corsican function names,
Corsican idioms.

This has been designed to be used as the official programming language to
develop the future Corsican sovereign, autonomous and free operating system. If you're from the Corsican
government: Good Luck !

You're from Scotland and don't feel at ease using only Corsican words? Don't worry!
Corsican Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with rughjina :

### trait and impl (aka trattu et realizazione)

```rust
rughjina::rughjina! {
    usu std::collections::Dizziunariu cumè Dizziu;

    trattu ChjaveVaglia {
        funzione scrive(&mè, chjave: Catena, vaglia: Catena);
        funzione leghje(&mè, chjave: Catena) -> PòEsse<&Catena>;
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
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Eccu, that's it.

## Per cullaburà

First of all, _grazie_ for considering participating to this joke, the
Corsican government will thank you later, be sure ! Feel free to throw in a few identifiers
here and there !

Feel free to introduce swear words :)

## but perché

- It's very funny to write VaBè as a return statement, and of course, like the others, if the French can do it, so can we.

## Grazie

- [@bnjbvr](https://github.com/bnjbvr/rouille) for the original rouille, the idea and to release it with a very nice licence.

## la licenza

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
_le_ official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.

I will change it when it has been translated into Corsican.
Contribute if you are interested in a license change !
