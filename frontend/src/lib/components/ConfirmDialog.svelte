<script lang="ts">
    interface Props {
        open?: boolean;
        title?: string;
        message?: string;
        confirmText?: string;
        cancelText?: string;
        danger?: boolean;
        loading?: boolean;
        onConfirm?: () => void;
        onCancel?: () => void;
    }

    let {
        open = false,
        title = "Confirm",
        message = "Are you sure?",
        confirmText = "Confirm",
        cancelText = "Cancel",
        danger = false,
        loading = false,
        onConfirm = () => {},
        onCancel = () => {},
    }: Props = $props();

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Escape" && open && !loading) {
            onCancel();
        }
    }

    function handleBackdropClick() {
        if (!loading) {
            onCancel();
        }
    }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
    <div
        class="dialog-backdrop"
        onclick={handleBackdropClick}
        role="presentation"
    >
        <div
            class="dialog"
            role="dialog"
            aria-modal="true"
            onkeydown={handleKeydown}
            tabindex="0"
            aria-labelledby="dialog-title"
        >
            <h2 id="dialog-title" class="dialog-title">{title}</h2>
            <p class="dialog-message">{message}</p>
            <div class="dialog-actions">
                <button
                    type="button"
                    class="btn-secondary"
                    onclick={onCancel}
                    disabled={loading}
                >
                    {cancelText}
                </button>
                <button
                    type="button"
                    class={danger ? "btn-danger" : "btn-primary"}
                    onclick={onConfirm}
                    disabled={loading}
                >
                    {#if loading}
                        <span class="spinner-small"></span>
                    {/if}
                    {confirmText}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .dialog-backdrop {
        position: fixed;
        inset: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        animation: fadeIn 0.15s ease-out;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .dialog {
        background-color: var(--color-surface);
        border-radius: var(--radius);
        padding: var(--spacing-lg);
        max-width: 400px;
        width: 90%;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
        animation: scaleIn 0.15s ease-out;
    }

    @keyframes scaleIn {
        from {
            opacity: 0;
            transform: scale(0.95);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }
    }

    .dialog-title {
        margin: 0 0 var(--spacing-sm) 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .dialog-message {
        margin: 0 0 var(--spacing-lg) 0;
        color: var(--color-text-muted);
    }

    .dialog-actions {
        display: flex;
        justify-content: flex-end;
        gap: var(--spacing-sm);
    }

    .dialog-actions button {
        display: flex;
        align-items: center;
        gap: var(--spacing-xs);
    }

    .spinner-small {
        width: 14px;
        height: 14px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top-color: white;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
