mod builder;

pub use crate::note::{link::Edge, Note, NoteId};
pub use builder::VaultBuilder;
use serde::Serialize;

use std::collections::HashMap;

/// A directed graph, where the nodes are [`Note`]s.
///
/// The graph is represented as an adjacency matrix. This gives fast lookup times and allows the
/// discovery of backlinks to be very quick.
#[derive(Default, Debug, Serialize, Clone)]
pub struct Vault {
    notes: HashMap<NoteId, Note>,
    id_lookup: HashMap<String, NoteId>,
}

impl Vault {
    /// An iterator visiting all pairs of IDs and corresponding notes in an arbitrary order.
    pub fn iter(&self) -> Notes {
        Notes {
            base: self.notes.iter(),
        }
    }

    /// An iterator visiting all pairs of IDs and corresponding notes in an arbitrary order, with
    /// mutable references to the notes.
    pub fn iter_mut(&mut self) -> NotesMut {
        NotesMut {
            base: self.notes.iter_mut(),
        }
    }

    /// Gets a reference to a [`Note`] by it's [`NoteId`].
    ///
    /// Returns the reference as `Some(note_ref)`. If the given [`NoteId`] does not correspond to
    /// any [`Notes`](Note), this function will return `None`.
    pub fn get(&self, id: &NoteId) -> Option<&Note> {
        self.notes.get(id)
    }

    /// Gets a mutable reference to a [`Note`] by it's [`NoteId`].
    ///
    /// Returns the reference as `Some`. If the given [`NoteId`] does not correspond to any
    /// [`Note`]s, this function will return `None`.
    pub fn get_mut(&mut self, id: &NoteId) -> Option<&mut Note> {
        self.notes.get_mut(id)
    }

    /// Returns the amount of [`Note`]s in the vault.
    pub fn len(&self) -> usize {
        self.notes.len()
    }

    /// Returns `true` if the vault does not contain any notes.
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Returns the [`NoteId`] to a note that has the supplied title or path.
    ///
    /// **Note**: If two notes share the same title, the [`NoteId`] returned is determined by which
    /// note is read last when generating the vault. This can make looking up IDs by title
    /// non-consistent. Paths are unique to each [`Note`] and should therefore be consistent.
    pub fn id_of(&self, title_or_path: impl AsRef<str>) -> Option<&NoteId> {
        self.id_lookup.get(title_or_path.as_ref())
    }

    /// Returns an iterator of backlink [`NoteId`]s.
    ///
    /// The IDs point to notes that link to the note identified by the paramater ID (`id`).
    /// Essentially, this gives you an iterator of
    /// [backlinks](https://en.wikipedia.org/wiki/Backlink).
    ///
    /// # Example
    ///
    /// This example prints all note titles and their backlinks.
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use mdzk::Vault;
    /// # let vault = Vault::default();
    /// for (id, note) in vault.iter() {
    ///     println!("Backlinks of {}:", note.title);
    ///     for backlink_id in vault.backlinks(*id) {
    ///         println!(" - {}", vault.get(backlink_id).unwrap().title);
    ///     }
    /// }
    /// ```
    pub fn backlinks(&self, id: NoteId) -> impl Iterator<Item = &NoteId> + '_ {
        self.iter()
            .filter_map(move |(backlink_id, note)| match note.adjacencies.get(&id) {
                Some(Edge::Connected) => Some(backlink_id),
                _ => None,
            })
    }
}

impl IntoIterator for Vault {
    type Item = (NoteId, Note);
    type IntoIter = std::collections::hash_map::IntoIter<NoteId, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}

impl PartialEq for Vault {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(id, note)| other.get(id).map_or(false, |n| *note == *n))
    }
}

impl Eq for Vault {}

/// An iterator returning references to all notes in a vault in an arbitrary order.
///
/// The notes are indexed by a reference to their [`NoteId`].
pub struct Notes<'a> {
    base: std::collections::hash_map::Iter<'a, NoteId, Note>,
}

impl<'a> Iterator for Notes<'a> {
    type Item = (&'a NoteId, &'a Note);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

pub struct NotesMut<'a> {
    base: std::collections::hash_map::IterMut<'a, NoteId, Note>,
}

/// An iterator returning mutable references to all notes in a vault in an arbitrary order.
///
/// The notes are indexed by a reference to their [`NoteId`].
impl<'a> Iterator for NotesMut<'a> {
    type Item = (&'a NoteId, &'a mut Note);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}
