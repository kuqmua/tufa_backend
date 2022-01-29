use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(GitInfoDerive)]
pub fn derive_git_info(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_git_info syn::parse(input) failed");
    let ident = &ast.ident;
    //must not panic
    //todo: write a message on start in case of error get config info
    //todo: make it parallel or async
    let gen = quote! {
        impl GitInfo for #ident {
            fn get_git_info(&self) -> String {
                let mut git_info = String::new();
                let commit_editmsg_string_path = ".git/COMMIT_EDITMSG";
                let commit_editmsg_path = Path::new(commit_editmsg_string_path);
                let commit_editmsg_name = "COMMIT_EDITMSG: ";
                match File::open(commit_editmsg_path) {
                    Err(e) => git_info.push_str(&format!("{}{:?}", commit_editmsg_name, e)),
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut commit_editmsg_content = String::new();
                        match buf_reader.read_to_string(&mut commit_editmsg_content) {
                            Err(e) => git_info.push_str(&format!("{}{:?}", commit_editmsg_name, e)),
                            Ok(_) => git_info.push_str(&format!(
                                "{}{}",
                                commit_editmsg_name, commit_editmsg_content
                            )),
                        }
                    }
                }
                let orig_head_string_path = ".git/ORIG_HEAD";
                let orig_head_path = Path::new(orig_head_string_path);
                let orig_head_name = "ORIG_HEAD: ";
                match File::open(orig_head_path) {
                    Err(e) => git_info.push_str(&format!("{}{:?}", orig_head_name, e)),
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut orig_head_content = String::new();
                        match buf_reader.read_to_string(&mut orig_head_content) {
                            Err(e) => git_info.push_str(&format!("{}{:?}", orig_head_name, e)),
                            Ok(_) => git_info.push_str(&format!("{}{}", orig_head_name, orig_head_content)),
                        }
                    }
                }
                let fetch_head_string_path = ".git/FETCH_HEAD";
                let fetch_head_path = Path::new(fetch_head_string_path);
                let fetch_head_name = "FETCH_HEAD: ";
                match File::open(fetch_head_path) {
                    Err(e) => git_info.push_str(&format!("{}{:?}", fetch_head_name, e)),
                    Ok(file) => {
                        let mut buf_reader = BufReader::new(file);
                        let mut fetch_head_content = String::new();
                        match buf_reader.read_to_string(&mut fetch_head_content) {
                            Err(e) => git_info.push_str(&format!("{}{:?}", fetch_head_name, e)),
                            Ok(_) => git_info.push_str(&format!("{}\n{}", fetch_head_name, fetch_head_content)),
                        }
                    }
                }
                git_info
            }
        }
    };
    gen.into()
}
