import { createClient } from '@supabase/supabase-js'
import type { Database } from '~/database.types';

const config = useRuntimeConfig()
export const useSupabase = createClient<Database>(config.supabaseUrl, config.supabaseKey, {
  auth: { autoRefreshToken: false, persistSession: false, detectSessionInUrl: false }
})

export default function () {
  return {
    ...useSupabase,
  }
}