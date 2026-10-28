import z from "zod"

export const signUpSchema = z.object({
  email: z.string().min(5, "Email must be at least 5 characters."),
  password: z.string().min(8, "Password must be at least 8 characters")
})

export type SignUpSchema = z.infer<typeof signUpSchema>
