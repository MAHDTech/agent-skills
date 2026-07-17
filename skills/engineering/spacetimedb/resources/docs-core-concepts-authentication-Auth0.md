Version: 2.0.0

On this page

This guide will walk you through integrating Auth0 authentication with
your SpacetimeDB application.

## Prerequisites

We assume you have the following prerequisites in place:

- A working SpacetimeDB project, follow our [React Quickstart
  Guide](/docs/quickstarts/react) if you need help setting this up.

## Getting started

**Install Auth0 React SDK**

Install the Auth0 React SDK into your React application.

- NPM
- Yarn
- PNPM
- Bun

``` codeBlockStandalone_LlrK
npm add @auth0/auth0-react
```

``` codeBlockStandalone_LlrK
yarn add @auth0/auth0-react
```

``` codeBlockStandalone_LlrK
pnpm add @auth0/auth0-react
```

``` codeBlockStandalone_LlrK
bun add @auth0/auth0-react
```

**Setup your Auth0 App**

1.  Head to the [Auth0 Dashboard](https://manage.auth0.com/dashboard/)
2.  Click on **Applications** \> **Applications** \> **Create
    Application**
3.  In the popup, enter a name for your app, select
    `Single Page Web Application` as the app type and click Create
4.  Switch to the **Settings** tab on the **Application Details** page
5.  Save the Domain and Client ID values from the dashboard somewhere
    handy, you'll need them later
6.  Finally, on the Settings tab of your Application Details page,
    configure the URLs from the table on the right

| URL Type              | URL                     |
|-----------------------|-------------------------|
| Allowed Callback URLs | `http://localhost:5173` |
| Allowed Logout URLs   | `http://localhost:5173` |
| Allowed Web Origins   | `http://localhost:5173` |

**Create an AutoLogin Component**

Create an `AutoLogin` component that automatically handles user login
and ID token retrieval using the Auth0 React SDK. This component will
redirect unauthenticated users to the Auth0 login page and provide the
ID token to its children via context.

``` codeBlockStandalone_LlrK
import { useAuth0 } from '@auth0/auth0-react';
import { createContext, useContext, useEffect, useMemo, useState } from 'react';

const IdTokenContext = createContext<string | undefined>(undefined);

export function useIdToken() {
  const ctx = useContext(IdTokenContext);
  if (!ctx) {
    throw new Error('useIdToken must be used within an IdTokenProvider');
  }
  return ctx;
}

export function AutoLogin({ children }: { children: React.ReactNode }) {
  const { isLoading, isAuthenticated, loginWithRedirect, getIdTokenClaims } =
    useAuth0();

  const [idToken, setIdToken] = useState<string | null>(null);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      loginWithRedirect().catch(err => setError(err));
    }
  }, [isLoading, isAuthenticated, loginWithRedirect]);

  useEffect(() => {
    let cancelled = false;

    const run = async () => {
      if (isLoading) return;

      // IMPORTANT: If not authenticated, ensure token is cleared
      if (!isAuthenticated) {
        if (!cancelled) setIdToken(null);
        return;
      }

      try {
        const claims = await getIdTokenClaims();
        const token = claims?.__raw ?? null;

        if (!token) {
          throw new Error('Auth0 returned no ID token (__raw missing).');
        }

        if (!cancelled) {
          setIdToken(token);
        }
      } catch (e) {
        if (!cancelled) setError(e as Error);
      }
    };

    run();
    return () => {
      cancelled = true;
    };
  }, [isLoading, isAuthenticated, getIdTokenClaims]);

  const value = useMemo<string | undefined>(() => {
    return idToken ?? undefined;
  }, [idToken]);

  const ready = !isLoading && isAuthenticated && !!idToken && !error;

  if (error) {
    return (
      <div>
        <p>Authentication error</p>
        <pre>{error.message}</pre>
      </div>
    );
  }

  if (!ready) {
    return <p>Loading...</p>;
  }

  return (
    <IdTokenContext.Provider value={value}>{children}</IdTokenContext.Provider>
  );
}
```

**Use Auth0Provider and AutoLogin in main.tsx**

Wrap your application with the `Auth0Provider` component to `main.tsx`
to enable authentication.

info

Don't forget to remove any existing `SpacetimeDBProvider` wrapper from
this file as we will be adding it somewhere else later in this guide.

``` codeBlockStandalone_LlrK
import { Auth0Provider } from '@auth0/auth0-react';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';
import { AutoLogin } from './AutoLogin.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Auth0Provider
      domain="<YOUR_AUTH0_DOMAIN>"
      clientId="<YOUR_AUTH0_CLIENT_ID>"
      authorizationParams={{
        redirect_uri: window.location.origin,
      }}
    >
      <AutoLogin>
        <App />
      </AutoLogin>
    </Auth0Provider>
  </StrictMode>
);
```

**Update App.tsx to use the ID Token**

Update your `App.tsx` file to use the `useIdToken` hook from the
`AutoLogin` component to get the ID token and pass it to the
`DbConnection` builder.

``` codeBlockStandalone_LlrK
import { useMemo } from 'react';
import { Identity } from 'spacetimedb';
import { SpacetimeDBProvider } from 'spacetimedb/react';
import { useIdToken } from './AutoLogin';
import { DbConnection, ErrorContext } from './module_bindings';

const onConnect = (_conn: DbConnection, identity: Identity) => {
  console.log(
    'Connected to SpacetimeDB with identity:',
    identity.toHexString()
  );
};

const onDisconnect = () => {
  console.log('Disconnected from SpacetimeDB');
};

const onConnectError = (_ctx: ErrorContext, err: Error) => {
  console.log('Error connecting to SpacetimeDB:', err);
};

export default function App() {
  const idToken = useIdToken();

  const connectionBuilder = useMemo(() => {
    return DbConnection.builder()
      .withUri('<YOUR SPACETIMEDB URL>')
      .withDatabaseName('<YOUR SPACETIMEDB MODULE NAME>')
      .withToken(idToken)
      .onConnect(onConnect)
      .onDisconnect(onDisconnect)
      .onConnectError(onConnectError);
  }, [idToken]);

  return (
    <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
      <div>
        <h1>SpacetimeDB React App</h1>
        <p>You can now use SpacetimeDB in your app!</p>
      </div>
    </SpacetimeDBProvider>
  );
}
```

- [Prerequisites](#prerequisites)
- [Getting started](#getting-started)
