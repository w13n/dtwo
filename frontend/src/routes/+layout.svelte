<script lang="ts">
    import "../app.css";
</script>

<header class="app-header">
    <div class="container header-content">
        <a href="/settings" class="logo">Settings Manager</a>
    </div>
</header>

<main>
    <slot />
</main>

<style>
    .app-header {
        background-color: var(--color-surface);
        border-bottom: 1px solid var(--color-border);
        position: sticky;
        top: 0;
        z-index: 100;
    }

    .header-content {
        display: flex;
        align-items: center;
        padding-top: var(--spacing-md);
        padding-bottom: var(--spacing-md);
    }

    .logo {
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--color-text);
        text-decoration: none;
    }

    .logo:hover {
        text-decoration: none;
        color: var(--color-primary);
    }

    main {
        min-height: calc(100vh - 60px);
    }
</style>
