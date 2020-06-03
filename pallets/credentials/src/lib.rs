#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	StorageMap,
	StorageValue,
	ensure
};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;
use codec::{Decode, Encode};
use pallet_timestamp as timestamp;
use pallet_balances as balances;



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


pub trait Trait: system::Trait + timestamp::Trait + balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct IdentityInfo {
	//pub issuer: AccountId,
	pub name: Vec<u8>,
	pub email: Vec<u8>,
	pub description: Vec<u8>,
	pub kyc_hash:Vec<u8>,
	pub additional: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SubjectDetails<AccountId> {
	pub issuer: AccountId,
	pub name: Vec<u8>,
	pub tags: Vec<u8>,
	pub description: Vec<u8>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, Default, PartialEq)]
pub struct Credential<Timestamp, AccountId> {
	subject: u32,
	when: Timestamp,
	by: AccountId,
}

decl_event! {
	pub enum Event < T >
	where
		AccountId = <T as system::Trait >::AccountId,
		//Hash = < T as system::Trait >::Hash,
		//Timestamp = < T as timestamp::Trait >::Moment,
	{
        CredentialIssued(AccountId, u32, AccountId),
        // A credential is revoked - holder, subj, issuer.
        CredentialRevoked(AccountId, u32, AccountId),
        // A new subject is created.
        SubjectCreated(AccountId, u32),
        //A new identity is created.
        IdentityCreated(AccountId),
	}
}


decl_storage! {

	trait Store for Module<T: Trait> as Credentials {

/*        // Global nonce for subject count.
        SubjectCount get(fn subject_count) config(): u32;
		// Issuers can issue credentials to others.
        // Issuer to Subject mapping.
        Subjects get(fn subjects) config(): map hasher(twox_64_concat) u32 => T::AccountId;

        // Credentials store.
        // Mapping (holder, subject) to Credential.
        Credentials get(fn credentials): map hasher(twox_64_concat) (T::AccountId, u32) => Credential<T::Moment, T::AccountId>;
        //credential manager
        CredManager get(fn cred_manager) config(): T::AccountId;

        //Reputation: default is 50.
        Reputation get(fn rep) config(): map hasher(twox_64_concat) T::AccountId => u32;

        //User map.
        Identities get(fn identities): map hasher(twox_64_concat) u64 => IdentityInfo;
        IdentityCount get(fn identity_count) : u64;
        IdentityIndex: map hasher(twox_64_concat) T::AccountId => u64;

        Nonce: u64;*/
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		//TODO
	}
}

decl_module! {
	pub struct Module < T: Trait > for enum Call where origin: T::Origin {
		type Error = Error < T >;
		fn deposit_event() = default;
	}
}

impl<T: Trait> Module<T> {
/*	pub fn do_issue(origin: T::Origin, to: T::AccountId, subject: u32) {
		// Check if origin is an issuer.
		// Issue the credential - add to storage.

		let sender = ensure_signed(origin)?;
		let subject_issuer = Self::Subjects(subject);
		//ensure!(sender == <CredManager<T>>::get(), "Unauthorized.");
		ensure!(subject_issuer == sender, "Unauthorized.");

		//ensure!(<Credentials<T>>::exists((to.clone(), subject)), "Credential already issued to user.");

		let now = <timestamp::Module<T>>::get();
		let cred = Credential {
			subject: subject,
			when: now,
			by: sender.clone()
		};

		<Credentials<T>>::insert((to.clone(), subject), cred);

		Self::deposit_event(RawEvent::CredentialIssued(to, subject, sender));
		Ok(());
	}*/
}