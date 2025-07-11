import { createClient } from '@supabase/supabase-js'
import type { Database } from '~/database.types';


export default function () {

  const config = useRuntimeConfig()
  console.log(process.env);

  const useSupabase = createClient<Database>(config.public.supabaseUrl, config.public.supabaseAnonKey)

  return {
    config,
    ...useSupabase,
  }
}