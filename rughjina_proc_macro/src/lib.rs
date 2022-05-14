use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Err" => "Err",
        "VaBè" => "Ok",
        "Catena" => "String",
        "Dizziunariu" => "HashMap",
        "Difettu" => "Default",
        "Errore" => "Error",
        "PòEsse" => "Option",
        "UnPocu" => "Some",
        "Nunda" => "None",
        "Risultatu" => "Result",
        "Mè" => "Self",
        "stampatuln" => "println",
        "stutà" => "break",
        "asincronu" => "async",
        "aspetta" => "await",
        "ricciuli" => "loop",
        "muvi" => "move",
        "cassa" => "crate",
        "codice_micca_accessibile" => "unreachable_code",
        "cumè" => "as",
        "custanti" => "const",
        "trattu" => "trait",
        "periculosu" => "unsafe",
        "di" => "in",
        "da" => "from",
        "dinamica" => "dyn",
        "sballà" => "unwrap",
        "difettu" => "default",
        "cumè_rif" => "as_ref",
        "es" => "io",
        "esternu" => "extern",
        "falsu" => "false",
        "funzione" => "fn",
        "perfettu" => "super",
        "inserì" => "insert",
        "ritruvà" => "get",
        "cuncedente" => "allow",
        "merda" | "cazzu" | "mancatu" => "panic",
        "modulu" => "mod",
        "mutabile" => "mut",
        "novu" => "new",
        "induve" => "where",
        "per" => "for",
        "piglià_o_inseritu_cun" => "get_or_insert_with",
        "principale" => "main",
        "publicu" => "pub",
        "riturna" => "return",
        "realizazione" => "impl",
        "rif" => "ref",
        "currisponde" => "match",
        "si" => "if",
        "sinnò" => "else",
        "mè" => "self",
        "lascia" => "let",
        "staticu" => "static",
        "struttura" => "struct",
        "prisume" => "expect",
        "finchì" => "while",
        "usu" => "use",
        "comu" => "into",
        "veru" => "true",
        "enumerazione" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rughjina(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
