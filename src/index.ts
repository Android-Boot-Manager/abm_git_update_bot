import { Hono } from 'hono'
import webhook from './routes/webhook'

const app = new Hono<{ Bindings: CloudflareBindings }>()

app.get('/', (c) => {
  return c.text('Hello Hono!')
})

app.route('/webhook', webhook)

export default app
