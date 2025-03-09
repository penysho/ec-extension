import { atom } from "jotai"

import { User } from "@/types/auth"

export const userAtom = atom<User | null>(null)
export const loadingAtom = atom<boolean>(false)
export const errorAtom = atom<string | null>(null)
export const pendingVerificationEmailAtom = atom<string | null>(null)
