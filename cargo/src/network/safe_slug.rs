use std::fmt;

use super::NetworkId;

/// The path segment the Safe transaction service uses for a network.
///
/// This is not the chain's `shortName`: seven of them disagree, Polygon being `matic` as a
/// short name but `pol` as a path segment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetworkSafeSlug(String);

impl NetworkSafeSlug {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        SLUGS
            .iter()
            .find(|(_, slug)| *slug == self.0)
            .map(|(id, _)| NetworkId(*id))
    }
}

impl NetworkId {
    pub fn safe_slug(self) -> Option<NetworkSafeSlug> {
        SLUGS
            .iter()
            .find(|(id, _)| *id == self.0)
            .map(|(_, slug)| NetworkSafeSlug((*slug).to_owned()))
    }
}

impl From<String> for NetworkSafeSlug {
    fn from(raw: String) -> Self {
        Self(raw)
    }
}

impl From<&str> for NetworkSafeSlug {
    fn from(raw: &str) -> Self {
        Self(raw.to_owned())
    }
}

impl fmt::Display for NetworkSafeSlug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

const SLUGS: &[(u64, &str)] = &[
    (1, "eth"),                   // Ethereum
    (10, "oeth"),                 // OP Mainnet
    (50, "xdc"),                  // XDC Network
    (56, "bnb"),                  // BNB Chain
    (100, "gno"),                 // Gnosis Chain
    (130, "unichain"),            // Unichain
    (137, "pol"),                 // Polygon
    (143, "monad"),               // Monad
    (146, "sonic"),               // Sonic
    (196, "okb"),                 // X Layer
    (204, "opbnb"),               // opBNB
    (324, "zksync"),              // zkSync Era
    (480, "wc"),                  // World Chain
    (677, "bot"),                 // BOT Chain Mainnet
    (988, "stable"),              // Stable
    (999, "hyper"),               // HyperEVM
    (1001, "kairos"),             // Kairos
    (1672, "pharos"),             // Pharos
    (1874, "wch-sepolia"),        // Whitechain Sepolia
    (2818, "morph"),              // Morph
    (3338, "peaq"),               // peaq
    (4217, "tempo"),              // Tempo
    (4326, "mega"),               // MegaETH
    (4663, "robinhood"),          // Robinhood Chain
    (5000, "mantle"),             // Mantle
    (5003, "mnt-sep"),            // Mantle Sepolia
    (5042, "arc"),                // Arc
    (8217, "kaia"),               // Kaia
    (8453, "base"),               // Base
    (9745, "plasma"),             // Plasma
    (10143, "monad-testnet"),     // Monad Testnet
    (10200, "chi"),               // Gnosis Chiado
    (16661, "0g"),                // 0G
    (25363, "fluent"),            // Fluent
    (36900, "adi"),               // ADI Network
    (42161, "arb1"),              // Arbitrum
    (42220, "celo"),              // Celo
    (42431, "tempo-moderato"),    // Tempo Moderato
    (43111, "hemi"),              // Hemi
    (43114, "avax"),              // Avalanche
    (46630, "robinhood-testnet"), // Robinhood Testnet
    (57073, "ink"),               // Ink
    (59144, "linea"),             // Linea
    (80069, "bep"),               // Bepolia
    (80094, "berachain"),         // Berachain
    (84532, "basesep"),           // Base Sepolia
    (102030, "ctc"),              // Creditcoin
    (534352, "scr"),              // Scroll
    (747474, "katana"),           // Katana
    (936485, "zenith-testnet"),   // Zenith Testnet
    (5042002, "arc-testnet"),     // Arc Testnet
    (11142220, "celo-sep"),       // Celo Sepolia Testnet
    (11155111, "sep"),            // Sepolia
    (1313161554, "aurora"),       // Aurora
];
