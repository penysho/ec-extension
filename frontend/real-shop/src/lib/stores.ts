import { atom } from "jotai"

import { User } from "@/hooks/useAuth"

export const userAtom = atom<User | null>(null)
export const loadingAtom = atom<boolean>(false)
export const errorAtom = atom<string | null>(null)
