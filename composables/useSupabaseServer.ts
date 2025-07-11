import { createClient } from '@supabase/supabase-js'
import type { Database } from '~/database.types';

const config = useRuntimeConfig()
console.log(config);

// export const useSupabase = createClient<Database>(config.public.supabaseUrl, config.supabaseKey, {
//   auth: { autoRefreshToken: false, persistSession: false, detectSessionInUrl: false }
// })

export default function () {
  return {
    // ...useSupabase,
  }
}