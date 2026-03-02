import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { auth } from './auth';

export async function middleware(request: NextRequest) {
  const session = await auth();

  // Protect /admin/* routes
  if (request.nextUrl.pathname.startsWith('/admin')) {
    if (!session) {
      return NextResponse.redirect(new URL('/signin', request.url));
    }

    // Check if token refresh failed
    if (session.error === 'RefreshAccessTokenError') {
      return NextResponse.redirect(new URL('/signin?error=TokenExpired', request.url));
    }

    // Check for admin role
    const hasAdminRole = session.user?.roles?.includes('admin');
    if (!hasAdminRole) {
      return NextResponse.redirect(new URL('/unauthorized', request.url));
    }
  }

  return NextResponse.next();
}

export const config = {
  matcher: ['/admin/:path*'],
};
