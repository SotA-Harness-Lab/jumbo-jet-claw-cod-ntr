//! Keeps safe display messages and bounded categories for local verification failures.

use codex_app_server_client::TypedRequestError;
use codex_app_server_protocol::UserVerificationCancellationReason;
use codex_app_server_protocol::UserVerificationErrorDetails;
use codex_app_server_protocol::UserVerificationFailureReason;
use codex_app_server_protocol::UserVerificationInvalidRequestReason;
use codex_app_server_protocol::UserVerificationUnavailableReason;

use crate::app_command::UserVerificationFailure;

pub(super) fn verification_failure(error: &TypedRequestError) -> UserVerificationFailure {
    let TypedRequestError::Server { source, .. } = error else {
        return UserVerificationFailure {
            message: crate::i18n::tr!("user-verification-could-not-complete-local"),
            details: UserVerificationErrorDetails::Failed {
                reason: UserVerificationFailureReason::ServiceError,
            },
        };
    };
    let details = source
        .data
        .clone()
        .and_then(|data| serde_json::from_value::<UserVerificationErrorDetails>(data).ok());
    let message = match details {
        Some(UserVerificationErrorDetails::InvalidRequest {
            reason: UserVerificationInvalidRequestReason::InvalidParams,
        }) => crate::i18n::tr!("user-verification-could-not-verify-request"),
        Some(UserVerificationErrorDetails::Unavailable {
            reason: UserVerificationUnavailableReason::CredentialMissing,
        }) => crate::i18n::tr!("user-verification-no-credential"),
        Some(UserVerificationErrorDetails::Unavailable {
            reason: UserVerificationUnavailableReason::BiometricsUnavailable,
        }) => crate::i18n::tr!("user-verification-biometric-unavailable"),
        Some(UserVerificationErrorDetails::Unavailable {
            reason: UserVerificationUnavailableReason::ProviderUnavailable,
        }) => crate::i18n::tr!("user-verification-unavailable-for-request"),
        Some(UserVerificationErrorDetails::Cancelled {
            reason: UserVerificationCancellationReason::UserCancelled,
        }) => crate::i18n::tr!("user-verification-cancelled"),
        Some(UserVerificationErrorDetails::Cancelled {
            reason: UserVerificationCancellationReason::Interrupted,
        }) => crate::i18n::tr!("user-verification-interrupted"),
        Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::AuthenticationFailed,
        }) => crate::i18n::tr!("user-verification-biometric-failed"),
        Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::Timeout,
        }) => crate::i18n::tr!("user-verification-timed-out"),
        Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::ProviderError,
        })
        | Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::ServiceError,
        })
        | None => crate::i18n::tr!("user-verification-could-not-complete"),
    };
    UserVerificationFailure {
        message,
        details: details.unwrap_or(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::ServiceError,
        }),
    }
}

#[cfg(test)]
#[path = "user_verification_errors_tests.rs"]
mod tests;
