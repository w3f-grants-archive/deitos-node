#![allow(unused_qualifications)]
use crate::*;
use frame_system::pallet_prelude::BlockNumberFor;

use scale_info::TypeInfo;

pub type BalanceOf<T> =
    <<T as Config>::Currency as FunInspect<<T as frame_system::Config>::AccountId>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
pub type StorageSizeMB = u64;
pub type AgreementTimeAllocation = u32;
pub type Installment<T> = BalanceOf<T>;
pub type PaymentsDetails<T> = (Installment<T>, PaymentPlanPeriods);
pub type PaymentPlan<T> = BoundedVec<PaymentsDetails<T>, <T as Config>::MaxPaymentPlanDuration>;
pub type ActiveAgreements<T> = BoundedVec<<T as Config>::AgreementId, <T as Config>::MaxAgreements>;

// TODO: Review the necessary status.
#[derive(Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo, Debug)]
pub enum IPStatus {
    Validating,
    NotReady,
    Active,
    Inactive,
    Suspended,
    Unregistered,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo, Debug)]
pub enum AgreementStatus {
    ConsumerRequest,
    IpAccepted,
    IpAcceptedWithModifications,
    IpRejected,
    ConsumerAcceptedModifications,
    Activated,
    Disputed,
    ConsumerBreach,
    Terminated,
    WaitingConsumerFeedback,
    WaitingProviderFeedback,
    Finished,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo, Debug)]
pub enum PaymentPlanPeriods {
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Semiannually,
    Annually,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
pub struct IPDetails<T: pallet::Config> {
    /// Total IP StorageSizeMB
    pub total_storage: StorageSizeMB,
    // StorageSizeMB already reserved by agreements
    pub reserved_storage: StorageSizeMB,
    // IP Status
    pub status: IPStatus,
    // Track of active agreements
    pub active_agreements: ActiveAgreements<T>,
    // Deposit funds
    pub deposit_amount: BalanceOf<T>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
pub struct AgreementDetails<T: pallet::Config> {
    pub id: T::AgreementId,
    // Agreement Status
    pub status: AgreementStatus,
    // Total amount of storage in the agreement expressed in bytes?
    pub storage: StorageSizeMB,
    // Amount of time the agreement is valid for in blocks
    pub time_allocation: AgreementTimeAllocation,
    // Activation block
    pub activation_block: BlockNumberFor<T>,
    // Payment plan
    pub payment_plan: PaymentPlan<T>,
    // Consumer agreement lock fee.
    pub agreement_lock_fee: BalanceOf<T>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
pub struct IPCostsPerUnit<T: pallet::Config> {
    // Price per block
    pub price_storage_per_block: BalanceOf<T>,
}
