// Simple CLI to (add, delete, update, create) i18n translation file
//     Copyright (C) 2020-2022  TheAwiteb
//     https://github.com/TheAwiteb/inrs
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod create;
mod delete;
mod list;
#[cfg(test)]
pub mod tests;
mod update;
mod utils;

pub mod errors;

use super::validator::validate_lang_name;
use clap::Subcommand;

pub use {
    create::create,
    delete::{delete_key, delete_language},
    list::list_translations,
    update::update,
    utils::Translation,
};

/// Delete sub commands
#[derive(Debug, Subcommand)]
pub enum DeleteSubCommands {
    /// Delete language from i18n directory 🗑️
    Lang {
        #[clap(short, long, value_parser = validate_lang_name)]
        lang: String,
    },
    /// Delete translation from languages 🗑️
    Trans {
        #[clap(short, long)]
        key: String,
    },
}

#[derive(Debug, Subcommand)]
/// Inrs sub commands
pub enum Subcommands {
    /// Create new language file 🔤
    Create {
        /// The language name 🔤
        #[clap(short, long, value_parser = validate_lang_name)]
        lang: String,
    },
    /// Add/Update translation 🆕
    Update {
        /// Language name to add/update in it 🆕
        #[clap(short, long, value_parser = validate_lang_name)]
        lang: String,
        /// The translation key 🗝
        #[clap(short, long)]
        key: String,
        /// The translation 🔤
        #[clap(short, long)]
        trans: String,
    },
    /// Delete translation/language 🚧
    Delete {
        #[clap(subcommand)]
        action: DeleteSubCommands,
    },
    /// List all translations for specific language 📊
    List {
        /// Language name to add/update in it 🆕
        #[clap(short, long, value_parser = validate_lang_name)]
        lang: String,
        /// Row width 📏
        #[clap(short, long, default_value = "40")]
        width: u16,
    },
}
