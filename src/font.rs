//! The monospace family for logs and reports, resolved against the system fonts.

/// Clash Nyanpasu's monospace stack, in the order of `--font-mono` in
/// `frontend/nyanpasu/src/assets/styles/tailwind.css`, so logs read like the
/// app's. CSS generic names stand for the system's choice.
const MONOSPACE_STACK: &[&str] = &[
    "Cascadia Mono",
    "Fira Code",
    "ui-monospace",
    "SFMono-Regular",
    "Menlo",
    "Monaco",
    "Consolas",
    "Liberation Mono",
    "Courier New",
    "Sarasa Mono SC",
    "monospace",
];

/// The first family of the monospace stack installed on this system, or an
/// empty string. Slint takes one family and falls back to a sans-serif font,
/// not a monospace one, when it is missing, so the stack is resolved here.
pub fn monospace_font() -> String {
    let mut collection = fontique::Collection::new(fontique::CollectionOptions {
        shared: false,
        system_fonts: true,
    });
    first_installed(MONOSPACE_STACK, |name| {
        let id = match fontique::GenericFamily::parse(name) {
            Some(generic) => collection.generic_families(generic).next(),
            None => collection.family_id(name),
        }?;
        Some(collection.family(id)?.name().to_owned())
    })
    .unwrap_or_default()
}

/// The family `resolve` finds for the first entry of `stack` that has one.
fn first_installed(stack: &[&str], resolve: impl FnMut(&str) -> Option<String>) -> Option<String> {
    stack.iter().copied().find_map(resolve)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_installed_family_of_the_stack_wins() {
        let installed = ["Consolas", "Menlo"];
        let resolved = first_installed(MONOSPACE_STACK, |name| {
            installed.contains(&name).then(|| name.to_owned())
        });
        assert_eq!(resolved.as_deref(), Some("Menlo"));
        assert_eq!(first_installed(MONOSPACE_STACK, |_| None), None);
    }

    #[test]
    fn this_system_has_a_monospace_family() {
        assert!(!monospace_font().is_empty());
    }
}
