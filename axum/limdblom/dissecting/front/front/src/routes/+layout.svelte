<script lang="ts">
	import '../app.css'
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { setLanguageTag, sourceLanguageTag, type AvailableLanguageTag } from '$paraglide/runtime';
	import { availableLanguageTags } from '$paraglide/runtime';
	import { route } from '$lib/i18n-routing';

	$: lang = ($page.params.lang as AvailableLanguageTag) ?? sourceLanguageTag;

	$: setLanguageTag(lang);
	// $: pathName = $page.url.pathname;
	if (browser && (!availableLanguageTags.includes($page.params.lang as AvailableLanguageTag))) {
		console.log("path name wrong");
		goto(`/en`);
	}
</script>

{#each availableLanguageTags as lang, i}
	<a href={route($page.url.pathname, lang)} hreflang={lang}>Change language to {lang}</a>
{/each}
<div>
	<a href={route("wasm",lang)}>To Wasm</a>
</div>
{#key lang}
	<slot />
{/key}
