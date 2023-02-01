/* A Macro Polo Game

if the name Macro is given, the program will respond with Polo.
Otherwise, the program will respond with "what's your name"
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}
