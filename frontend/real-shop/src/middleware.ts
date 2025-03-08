import { NextRequest, NextResponse } from "next/server"

import { USER_ID_COOKIE_NAME } from "./types/auth"

export function middleware(req: NextRequest) {
  const userId = req.cookies.get(USER_ID_COOKIE_NAME)?.value

  if (!userId) {
    return NextResponse.redirect(new URL("/login", req.url))
  }

  return NextResponse.next()
}

export const config = {
  matcher: ["/account/:path*"],
}
