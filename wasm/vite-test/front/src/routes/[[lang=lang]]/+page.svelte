<script lang="ts">
	import { availableLanguageTags } from '$paraglide/runtime';
	import * as m from '$paraglide/messages';
	import { page } from '$app/stores';
	import { route } from '$lib/i18n-routing';
</script>

<p class="bg-slate-500 text-red-200">{m._test()}</p>
<p class="text-red-200 bg-slate-500">{m._test()}</p>
