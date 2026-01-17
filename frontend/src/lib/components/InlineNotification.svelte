<script lang="ts">
    import { onMount } from "svelte";
    import type { Notification } from "$lib/types";

    export let notification: Notification | null = null;
    export let autoDismissMs: number = 4000;

    let visible = false;
    let timeoutId: ReturnType<typeof setTimeout> | null = null;

    $: if (notification) {
        visible = true;
        if (timeoutId) clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            visible = false;
            notification = null;
        }, autoDismissMs);
    }

    onMount(() => {
        return () => {
            if (timeoutId) clearTimeout(timeoutId);
        };
    });
</script>

{#if visible && notification}
    <div class="notification notification-{notification.type}" role="alert">
        <span class="notification-icon">
            {#if notification.type === "success"}
                ✓
            {:else}
                ✕
            {/if}
        </span>
        <span class="notification-message">{notification.message}</span>
    </div>
{/if}

<style>
    .notification {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
        padding: var(--spacing-sm) var(--spacing-md);
        border-radius: var(--radius);
        margin-top: var(--spacing-md);
        animation: slideIn 0.2s ease-out;
    }

    @keyframes slideIn {
        from {
            opacity: 0;
            transform: translateY(-10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .notification-success {
        background-color: #d4edda;
        border: 1px solid #c3e6cb;
        color: #155724;
    }

    .notification-error {
        background-color: #f8d7da;
        border: 1px solid #f5c6cb;
        color: #721c24;
    }

    .notification-icon {
        font-weight: bold;
        font-size: 1rem;
    }

    .notification-message {
        flex: 1;
    }
</style>
