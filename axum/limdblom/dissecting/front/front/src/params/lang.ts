// ./src/params/lang.ts
import { availableLanguageTags } from "$paraglide/runtime"
import type { AvailableLanguageTag } from "$paraglide/runtime"

export const match = (param: any): param is AvailableLanguageTag => {
	// console.log("match", param, availableLanguageTags)
	return availableLanguageTags.includes(param)
}
