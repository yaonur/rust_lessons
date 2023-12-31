<script>
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    let availableLanguages = ['en','tr', 'es', 'fr'];
    let selectedLanguage = $page.params.lang || 'en';
    console.log($page.route.id)

    // function changeLanguage() {
    //     const pathSegments = $page.route.id?.split('/') ;
    //     const newPath = `/${selectedLanguage}/${pathSegments?.slice(2).join('/')}`;
    //     goto(newPath);
    // }
</script>

<div class="flex justify-between {$$props.class}">
    <nav>
        <ul class="flex gap-2 bg-gray-300">
            <li><a href="/en">Home</a></li>
            <li>
                <a href="/en/uploadvideo">Upload</a>
            </li>
            <li>
                <a href="/en/login">Login</a>
            </li>
        </ul>
    </nav>
    <!-- <select bind:value={selectedLanguage} on:change={changeLanguage}>
        {#each availableLanguages as lang}
            <option value={lang}>{lang}</option>
        {/each}
    </select> -->
</div>