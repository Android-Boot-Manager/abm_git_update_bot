import { Hono } from 'hono'

const app = new Hono()

app.post('/', async (c) => {
  const { text } = await c.req.parseBody()
  if (typeof text !== 'string') {
    return c.text('Forbidden', 403)
  }

  return c.text('Forbidden', 403)
})

export default app
