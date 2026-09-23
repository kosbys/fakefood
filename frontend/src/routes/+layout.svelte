<script lang="ts">
    import favicon from "$lib/assets/favicon.svg";
    import Footer from "$lib/components/Footer.svelte";
    import Header from "$lib/components/Header.svelte";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import "../app.css"

    let { children } = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>


<Sidebar>
    <Header></Header>
    {@render children()}
</Sidebar>
<Footer></Footer>
