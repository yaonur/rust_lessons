// ./src/lib/i18n-routing.ts
import {
	sourceLanguageTag,
	type AvailableLanguageTag,
	availableLanguageTags,
} from "$paraglide/runtime"

/**
 * Returns the path in the given language, regardless of which language the path is in.
 */
export function route(path: string, lang: AvailableLanguageTag) {
	const pathProcessed = withoutLanguageTag(path,lang)
	
	return `/${lang}/${pathProcessed}`
}

/**
 * Returns the path without the language tag
 */
function withoutLanguageTag(path: string,lang: string) {
	const [_, maybeLang, ...rest] = path.split("/")
	if (availableLanguageTags.includes(maybeLang as AvailableLanguageTag)) {
		return rest.join("/")
	}
	return path
}
