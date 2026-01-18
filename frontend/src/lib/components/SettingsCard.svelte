<script lang="ts">
    import type { Settings } from "$lib/types";
    import { getSettingData } from "$lib/helpers";

    interface Props {
        settings: Settings;
        onDelete: (id: string) => void;
    }

    let { settings, onDelete }: Props = $props();

    let displayData = $derived(getSettingData(settings));
    let previewJson = $derived(JSON.stringify(displayData, null, 2));
    let truncatedPreview =
        $derived(previewJson.length > 200
            ? previewJson.slice(0, 200) + "..."
            : previewJson);
</script>

<div class="settings-card">
    <div class="settings-header">
        <code class="settings-id">{settings.id}</code>
        <div class="settings-actions">
            <a href="/settings/{settings.id}/edit" class="btn-link">Edit</a>
            <button
                type="button"
                class="btn-link btn-link-danger"
                onclick={() => onDelete(settings.id)}
            >
                Delete
            </button>
        </div>
    </div>
    <pre class="settings-preview">{truncatedPreview}</pre>
</div>

<style>
    .settings-card {
        background-color: var(--color-surface);
        border: 1px solid var(--color-border);
        border-radius: var(--radius);
        padding: var(--spacing-md);
        transition: box-shadow 0.2s ease;
    }

    .settings-card:hover {
        box-shadow: 0 2px 8px var(--color-shadow);
    }

    .settings-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: var(--spacing-sm);
    }

    .settings-id {
        font-family: var(--font-mono);
        font-size: 0.875rem;
        color: var(--color-text-muted);
        background-color: var(--color-bg);
        padding: var(--spacing-xs) var(--spacing-sm);
        border-radius: var(--radius);
    }

    .settings-actions {
        display: flex;
        gap: var(--spacing-sm);
    }

    .btn-link {
        background: none;
        padding: var(--spacing-xs) var(--spacing-sm);
        color: var(--color-primary);
        font-size: 0.875rem;
    }

    .btn-link:hover {
        text-decoration: underline;
    }

    .btn-link-danger {
        color: var(--color-danger);
    }

    .settings-preview {
        margin: 0;
        padding: var(--spacing-sm);
        background-color: var(--color-bg);
        border-radius: var(--radius);
        font-family: var(--font-mono);
        font-size: 0.8125rem;
        overflow-x: auto;
        white-space: pre-wrap;
        word-break: break-word;
    }
</style>
