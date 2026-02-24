#![allow
(
    static_mut_refs,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    unused_variables,
)]
/*
\s+ret
(\s+.+){1}:
(\s+.+){1}
\s+ret
*/
/*
start:
    time_ok:
    display_bytes_count:
*/
#[unsafe(no_mangle)] pub unsafe fn start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        match rcx
        {
            0 =>
            {
                return ( rcx, rdx, r8, r9 )
            }
            
            _ =>
            {
                return ( rcx, rdx, r8, r9 )
            }
        }
        
        return ( rcx, rdx, r8, r9 )
    }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn information( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        match rcx
        {
            0 =>
            {
                return ( rcx, rdx, r8, r9 )
            }

            _ =>
            {
                return ( rcx, rdx, r8, r9 )
            }
        }

        return ( rcx, rdx, r8, r9 )
    }
}
/*
get_params:
    get_param:
    get_output_file:
    option_param:
    bad_params:
    memory_option:
    get_memory_setting:
    passes_option:
    get_passes_setting:
    next_param:
    definition_option:
    get_definition:
    symbols_option:
    get_symbols_setting:
    get_option_value:
    get_option_digit:
    option_value_ok:
    invalid_option_value:
    convert_definition_option:
    copy_definition_name:
    bad_definition_option:
    copy_definition_value:
    definition_value_end:
*/
#[unsafe(no_mangle)] pub unsafe fn get_params( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
collect_path
    copy_path_to_low_memory
*/
#[unsafe(no_mangle)] pub unsafe fn collect_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
init_memory
    allocate_memory:
    high_brk:
    mmap_unusable:
    mmap_with_hint:
    no_low_memory:
    mmap_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn init_memory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn exit_program( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
get_environment_variable:
    next_variable:
    compare_variable_names:
    compare_character:
    no_environment_variable:
    end_of_variable_name:
    copy_variable_value:
*/
#[unsafe(no_mangle)] pub unsafe fn get_environment_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
open:
    adapt_path:
    copy_path:
    path_char_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn open( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
create:
    do_create:
*/
#[unsafe(no_mangle)] pub unsafe fn create( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn close( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
read:
    file_error:
*/
#[unsafe(no_mangle)] pub unsafe fn read( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn write( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn display_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
display_block:
    block_displayed:
*/
#[unsafe(no_mangle)] pub unsafe fn display_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn display_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
display_number:
    display_loop:
    display_digit:
    digit_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn display_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
display_user_messages:
    line_break_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn display_user_messages( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/**/
#[unsafe(no_mangle)] pub unsafe fn fatal_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
assembler_error:
    get_error_lines:
    find_definition_origin:
    get_next_error_line:
    display_error_line:
     line_number_ok:
     get_line_data:
     display_line_data:
     line_data_displayed:
     convert_instruction:
     copy_symbol:
     space_ok:
     quoted:
     copy_quoted:
     quote_ok:
     quoted_copied:
     instruction_converted:
     display_error_message:
*/
#[unsafe(no_mangle)] pub unsafe fn assembler_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_timestamp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn out_of_memory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn stack_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn main_file_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_failed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn code_cannot_be_generated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_limitations_exceeded( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
invalid_definition:
    general_error:
*/
#[unsafe(no_mangle)] pub unsafe fn invalid_definition( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn file_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn error_reading_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_file_format( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_macro_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn incomplete_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unexpected_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn illegal_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_operand( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_operand_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operand_size_not_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operand_sizes_do_not_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_address_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn address_sizes_do_not_agree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn disallowed_combination_of_registers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn long_immediate_not_encodable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn relative_jump_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn value_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
undefined_symbol:
    copy_asciiz:
    write_quoted_symbol_name:
*/
#[unsafe(no_mangle)] pub unsafe fn undefined_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
symbol_out_of_scope:
    finish_symbol_out_of_scope_message:
*/
#[unsafe(no_mangle)] pub unsafe fn symbol_out_of_scope( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_use_of_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn name_too_long( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserved_word_used_as_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_already_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn missing_end_quote( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn missing_end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unexpected_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn extra_characters_on_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_not_aligned_enough( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn setting_already_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_already_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn too_many_repeats( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assertion_failed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
invoked_error:
    error_with_source:
*/
#[unsafe(no_mangle)] pub unsafe fn invoked_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
dump_symbols:
     prepare_strings_table:
     prepare_string:
     prepare_external_string:
     prepare_section_string:
     copy_elf_section_name:
     prepare_default_section_string:
     strings_table_ready:
     prepare_labels_dump:
     label_name_outside_source:
     label_dump_name_ok:
     label_dump_line_ok:
     convert_base_symbol_for_label:
     base_symbol_for_label_ok:
     label_defined_flag_ok:
     label_used_flag_ok:
     labels_dump_ok:
     make_lines_dump:
     process_line_dump:
     store_offset:
     base_symbol_for_line_ok:
     lines_dump_ok:
     find_inexisting_offsets:
     correct_inexisting_offset:
     write_symbols:
     make_references_dump:
     dump_reference:
     references_dump_ok:
     setup_dump_header:
*/
#[unsafe(no_mangle)] pub unsafe fn dump_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
prepare_preprocessed_source:
    prepare_preprocessed_line:
    line_not_from_main_input:
    prepare_next_preprocessed_line:
    preprocessed_source_ok:
    skip_preprocessed_line:
    skip_preprocessed_line_content:
    skip_preprocessed_string:
    skip_preprocessed_symbol:
*/
#[unsafe(no_mangle)] pub unsafe fn prepare_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
restore_preprocessed_source:
     restore_preprocessed_line:
     preprocessed_line_source_restored:
     restore_next_preprocessed_line:
     preprocessed_source_restored:
*/
#[unsafe(no_mangle)] pub unsafe fn restore_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn dump_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}
/*
preprocessor:
     make_characters_table:
     mark_symbol_characters:
     process_predefinitions:
     convert_predefinition:
     predefinition_symbol:
     found_predefinition_separator:
     predefinition_separator:
     predefinition_string:
     copy_predefinition_string:
     predefinition_backslash:
     group_predefinition_backslashes:
     predefinition_backslashed_symbol:
     convert_predefinition_backslashed_symbol:
     predefinition_backslashed_symbol_character:
     predefinition_converted:
     predefinitions_ok:
     process_postponed:
     find_postponed_list:
     process_postponed_list:
     find_earliest_postponed:
     earliest_postponed_found:
     preprocessing_finished:
     use_postponed_macro:
*/
#[unsafe(no_mangle)] pub unsafe fn preprocessor( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            match rcx
            {
                0 =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }

                _ =>
                    {
                        return ( rcx, rdx, r8, r9 )
                    }
            }

            return ( rcx, rdx, r8, r9 )
        }
}

pub mod api
{
    use std::arch::{ * };
   /*
    Monotonic Functions Grokked From FASM
    skip_match_ok     | add	esi, eax */
    pub unsafe fn add_from_the_to_the_source( mut rcx:usize, rdx:usize, r8:usize, r9:usize ) -> ( usize, usize, usize, usize )
    {
        unsafe
        {
            asm!
            (
                r#""#,
                inlateout("esi") rcx=>rcx,
            );

            (rcx, rdx, r8, r9)   
        }
    }
}

pub mod env
{
    pub use std::env::{ * };
}

pub mod mem
{
    pub use std::mem::{ * };
}

pub static mut ARGUMENTS:Vec<&'static str> = Vec::new();

pub unsafe fn flat( rcx:usize, rdx:usize, r8:usize, r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        let ( rcx, rdx, r8, mut r9 ) = api::add_from_the_to_the_source( rcx+5, rdx, r8, r9 );
        println!( r#"{:?}"#, rcx );
        r9 = 5;
        ( rcx, rdx, r8, r9 )
    }
}

pub unsafe fn domain( from:() )
{
    unsafe
    {
        if ARGUMENTS.len() > 1 
        {
            for argument in ARGUMENTS.clone()
            {
                println!( r#"{:?}"#, argument );
            }
        }

        let ( rcx, rdx, r8, r9 ) = flat( 0, 0, 0, 0 );
    }
}

pub fn main()
{
    unsafe
    {
        
        let mut arguments = env::args().collect::<Vec<String>>();
        
        if arguments.len() > 1
        {   
            for argument in arguments.clone()
            {
                let arg = argument.clone();
                ARGUMENTS.push( mem::transmute( arg.as_str() ) );
            }
        }

        domain( arguments = vec![] );
    }
}
/*
1759 */
