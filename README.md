# Shorty

Shorty is a simple url self-hosted url shortener. Its not ment to be used in a
public environment.

## Motivation

I was looking at self-hosted url shorteners but could not find any that
required little to no setup. After being tired with the speed of YOURLS. I
decided to write my own.

# Running

## Using docker compose

Clone the project and run `docker compose up` to startup the service. This
creates a postgres instance and a shorty instance. You'll probably need to
modify the environment section for shorty to you're likings.

## Environment variables

The following environment variables are required to startup the service:

- `SHORTY_HOST`: The host to which the service should bind to.
- `SHORTY_PORT`: The port the service should run on.
- `SHORTY_VISIBLE_HOST`: An url from which shorty is visible. This is used to
  create the redirect urls.
- `SHORTY_DATABASE_URL`: A database url to a postgres database.

# Endpoint

## API

Shorty opens two endpoints:

### `POST /s`

Shorten a url. The request expected a json object with the following structure:

```jsonc
{
  // Url to shorten
  "url": "https://github.com/nils-degroot/shorty"
}
```

The shortened url is return in plain-text in the response.

### `GET /s/{short}`

Redirect the user to the provided `short`. This is obtained from the `POST /s`
endpoint.

## UI

To reach the short ui, visit the `/ui` endpoint on your shorty instance. Here
you'd be able to shorten url's with a UI.
