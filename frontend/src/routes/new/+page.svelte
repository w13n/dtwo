<script lang="ts">
    import { goto } from "$app/navigation";
    import { enhance } from "$app/forms";
    import { createSettings, ApiClientError } from "$lib/api";
    import type { Notification } from "$lib/types";
    import JsonEditor from "$lib/components/JsonEditor.svelte";
    import InlineNotification from "$lib/components/InlineNotification.svelte";

    let jsonValue = $state("{}");
    let jsonError: string | null = $state(null);
    let loading = $state(false);
    let notification: Notification | null = $state(null);

    async function handleSubmit() {
        if (jsonError) {
            notification = {
                type: "error",
                message: "Please fix the JSON errors before saving",
            };
            return;
        }

        loading = true;
        notification = null;

        try {
            const data = JSON.parse(jsonValue);
            await createSettings(data);
            notification = {
                type: "success",
                message: "Settings created successfully",
            };
            setTimeout(() => goto("/"), 1000);
        } catch (e) {
            if (e instanceof ApiClientError) {
                notification = { type: "error", message: e.message };
            } else if (e instanceof SyntaxError) {
                notification = {
                    type: "error",
                    message: "Invalid JSON format",
                };
            } else {
                notification = {
                    type: "error",
                    message: "Failed to create settings",
                };
            }
        } finally {
            loading = false;
        }
    }

    function handleCancel() {
        goto("/");
    }
</script>

<svelte:head>
    <title>Create Settings - Settings Manager</title>
</svelte:head>

<div class="container">
    <div class="page-header">
        <h1>Create Settings</h1>
    </div>

    <div class="card">
        <form onsubmit={handleSubmit} use:enhance>
            <div class="form-group">
                <label for="json-editor">Settings Data (JSON)</label>
                <JsonEditor bind:value={jsonValue} bind:error={jsonError} />
            </div>

            <InlineNotification bind:notification />

            <div class="form-actions">
                <button
                    type="button"
                    class="btn-secondary"
                    onclick={handleCancel}
                    disabled={loading}
                >
                    Cancel
                </button>
                <button
                    type="submit"
                    class="btn-primary"
                    disabled={loading || !!jsonError}
                >
                    {#if loading}
                        <span class="spinner-inline"></span>
                        Creating...
                    {:else}
                        Create Settings
                    {/if}
                </button>
            </div>
        </form>
    </div>
</div>

<style>
    .btn-primary {
        display: inline-flex;
        align-items: center;
        gap: var(--spacing-xs);
    }

    .spinner-inline {
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
