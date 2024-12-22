import { NextRequest, NextResponse } from "next/server"

export function middleware(req: NextRequest) {
  const userId = req.cookies.get("userId")?.value

  if (!userId) {
    return NextResponse.redirect(new URL("/login", req.url))
  }

  return NextResponse.next()
}

export const config = {
  matcher: ["/account/:path*"],
}
