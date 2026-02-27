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
/*
*/
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
/*
preprocess_file:
    preprocess_source:
    next_line:
    file_end:
*/
#[unsafe(no_mangle)] pub unsafe fn preprocess_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_line:
     convert_line_data:
     convert_symbol:
     found_separator:
     convert_separator:
     symbol_character:
     control_character:
     lf_character:
     cr_character:
     convert_string:
     copy_string:
     backslash_character:
     group_backslashes:
     no_end_quote:
     backslashed_symbol:
     convert_backslashed_symbol:
     backslashed_symbol_character:
     concatenate_lines:
     find_concatenated_line:
     concatenate_lf:
     concatenate_cr:
     concatenate_ok:
     ignore_comment:
     line_end:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
lower_case:
    convert_case:
    case_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn lower_case( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_directive:
    scan_directives:
    next_directive:
    no_directive:
    directive_found:
    directive_handler:
    get_directive_handler_base:
*/
#[unsafe(no_mangle)] pub unsafe fn get_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
preprocess_line:
    preprocess_current_line:
    line_start_ok:
    not_fix_constant:
    macro_preprocessing:
    initial_preprocessing_ok:
    preprocess_instruction:
    not_preprocessor_directive:
    not_macro:
    not_symbolic_constant:
    struc_name_ok:
    preprocess_label:
    symbolic_constant_in_label:
    check_for_broken_label:
    label_broken:
    label_constant_ok:
    move_rest_of_line_up:
    replace_label:
    not_preprocessor_symbol:
    line_preprocessed:
*/
#[unsafe(no_mangle)] pub unsafe fn preprocess_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_preprocessor_symbol:
    preprocessor_special_symbol_not_recognized:
    no_preprocessor_special_symbol:
    follow_hashes_roots:
    follow_hashes_tree:
    compare_with_preprocessor_symbol:
    next_equal_hash:
    preprocessor_symbol_not_found:
    preprocessor_symbol_found:
    calculate_hash:
    fnv1a_hash:
*/
#[unsafe(no_mangle)] pub unsafe fn get_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
add_preprocessor_symbol:
    preprocessor_symbol_name_ok:
    reshape_hash:
    find_leave_for_symbol:
    find_entry_to_reuse:
    add_symbol_entry:
    reuse_symbol_entry:
    extend_hashes_tree:
*/
#[unsafe(no_mangle)] pub unsafe fn add_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn define_fix_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
define_equ_constant:
    define_preprocessor_constant:
*/
#[unsafe(no_mangle)] pub unsafe fn define_equ_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn define_symbolic_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn define_struc( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
define_macro:
    make_macro:
    skip_macro_arguments:
    skip_macro_argument:
    macro_argument_end:
    macro_arguments_finisher:
    end_macro_arguments:
    macro_argument_with_default_value:
    skip_macro_argument_value:
    enclosed_argument:
    enclosed_symbol:
    enclosed_string:
    enclosed_argument_end:
    simple_argument:
    argument_symbol:
    argument_string:
    argument_value_end:
    find_macro_block:
    found_macro_block:
    skip_macro_block:
    skip_macro_symbol:
    skip_macro_string:
*/
#[unsafe(no_mangle)] pub unsafe fn define_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn postpone_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn rept_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn irp_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn irps_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn irpv_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn match_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
define_instant_macro:
    skip_parameters:
    skip_quoted_parameter:
    parameters_skipped:
*/
#[unsafe(no_mangle)] pub unsafe fn define_instant_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
prepare_match:
    skip_pattern:
    skip_symbol_in_pattern:
    skip_quoted_string_in_pattern:
    pattern_skipped:
*/
#[unsafe(no_mangle)] pub unsafe fn prepare_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn purge_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn purge_struc( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
restore_equ_constant:
    restore_preprocessor_symbol:
    no_symbol_to_restore:
    symbol_restored:
*/
#[unsafe(no_mangle)] pub unsafe fn restore_equ_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn process_fix_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
process_equ_constants:
    process_symbolic_constants:
    ignore_string:
    check_brace:
    no_replacing:
    check_symbol:
    replace_symbolic_constant:
    process_after_replaced:
    move_data:
    movsb_ok:
    movsw_ok:
    string_after_replaced:
    brace_after_replaced:
    symbol_after_replaced:
    replace_special_symbolic_constant:
    preprocessed_file_value:
    preprocessed_line_value:
    get_current_line_from_file:
    find_line_from_file:
    line_from_file_found:
*/
#[unsafe(no_mangle)] pub unsafe fn process_equ_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
process_macro_operators:
    before_macro_operators:
    no_more_macro_operators:
    symbol_before_macro_operators:
    symbol_before_macro_operators_ok:
    string_before_macro_operators:
    escaped_symbol:
    reduce_symbol_conversion:
    symbol_conversion:
    symbol_character_conversion:
    convert_to_quoted_string:
    shift_line_data:
    concatenation:
    no_concatenation:
    symbol_concatenation:
    do_symbol_concatenation:
    concatenate_escaped_symbol:
    string_concatenation:
    concatenate_converted_symbol:
    finish_concatenating_converted_symbol:
    concatenate_converted_symbol_character:
    do_string_concatenation:
    after_macro_operators:
    symbol_after_macro_operators:
    symbol_after_macro_operatorss_ok:
    string_after_macro_operators:
*/
#[unsafe(no_mangle)] pub unsafe fn process_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
use_macro:
    process_macro_arguments:
    get_macro_arguments:
    next_argument:
    next_arguments_group:
    get_macro_argument:
    get_default_value:
    required_value:
    default_value_ok:
    greedy_macro_argument:
    got_macro_argument:
    macro_argument_ok:
    finish_macro_argument:
    argument_value_length_ok:
    arguments_end:
*/
#[unsafe(no_mangle)] pub unsafe fn use_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
use_instant_macro:
    add_rept_counter:
    rept_counter_added:
    rept_counters_ok:
    instant_macro_parameters_ok:
    instant_macro_finish:
    instant_macro_done:
    instant_macro_attached_line:
    precalculate_value:
    value_precalculated:
*/
#[unsafe(no_mangle)] pub unsafe fn use_instant_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
do_irp:
    irp_name_ok:
    irp_with_default_value:
    irps_name_ok:
    irp_parameters_start:
    get_irp_parameter:
    irp_parameters_end:
    get_irps_parameter:
    irps_quoted_string:
    irps_symbol:
    irps_parameter_ok:
    irps_parameters_end:
    get_irpv_parameter:
    mark_variable_value:
    next_variable_value:
    variable_values_marked:
    add_irpv_value:
    collect_next_variable_value:
    variable_values_collected:
*/
#[unsafe(no_mangle)] pub unsafe fn do_irp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
do_match:
    free_match:
    find_exact_match:
    try_different_matching:
    match_more_elements:
    skip_match_element:
    skip_match_quoted_string:
    skip_match_symbol:
    skip_match_ok:
    cannot_match:
    exact_match:
    exact_match_complete:
    match_verbatim:
    match_elements:
    symbol_characters_matched:
    match_quoted_strings:
    match_symbols:
    compare_elements:
    elements_mismatch:
    end_matching:
    matched_pattern:
    add_matched_symbol:
    matched_symbols_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn do_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
process_macro:
     find_macro_instructions:
     macro_instructions_start:
     process_macro_line:
     instant_macro_line_header:
     find_defining_directive:
     defining_directive_ok:
     macro_line_header_ok:
     process_macro_line_element:
     copy_macro_string:
     process_macro_symbol:
     process_macro_directive:
     not_macro_directive:
     replace_macro_symbol:
     group_macro_symbol:
     multiple_macro_symbol_values:
     enclose_macro_symbol_value:
     macro_symbol_value_ok:
     multiple_macro_symbol_values_ok:
     replace_macro_counter:
     group_macro_counter:
     multiple_macro_counter_values:
     store_number_symbol:
     numer_symbol_sign_ok:
     store_number_digits:
     store_number_digit:
     number_digit_ok:
     not_macro_symbol:
     copy_raw_symbol:
     copy_struc_name:
     disable_replaced_struc_name:
     skip_foreign_symbol:
     skip_foreign_line:
     skip_foreign_string:
     macro_foreign_line:
     macro_line_processed:
     process_next_line:
     macro_block_processed:
*/
#[unsafe(no_mangle)] pub unsafe fn process_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
local_symbols:
    make_local_symbol:
    counter_loop:
    letter_digit:
    small_letter_digit:
    counter_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn local_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn common_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn forward_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
reverse_block:
     new_macro_block:
*/
#[unsafe(no_mangle)] pub unsafe fn reverse_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
close_macro_block:
     reverse_counter:
     continue_block:
     block_closed:
*/
#[unsafe(no_mangle)] pub unsafe fn close_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_macro_symbol:
     try_macro_symbol:
     macro_symbol_found:
     macro_symbol_not_found:
     find_macro_symbol_leaf:
     follow_macro_symbols_tree:
     no_such_macro_symbol:
*/
#[unsafe(no_mangle)] pub unsafe fn get_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
add_macro_symbol:
     make_macro_symbol:
     extend_macro_symbol_tree:
*/
#[unsafe(no_mangle)] pub unsafe fn add_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
include_file:
     find_current_file_path:
     copy_current_file_path:
     cut_current_file_name:
     current_file_path_ok:
     try_include_directories:
     try_in_current_directory:
     include_path_ok:
     copy_preprocessed_path:
*/
#[unsafe(no_mangle)] pub unsafe fn include_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
parser:
     parser_loop:
     parse_line:
     no_data_label:
     simple_label:
     block_label:
     constant_label:
     data_label:
     main_instruction_identified:
     common_parse:
     empty_instruction:
     empty_line:
     skip_rest_of_line:
     parse_next_line:
     source_parsed:
     blocks_stack_ok:
     parse_block:
     parse_end_directive:
     parse_end_block:
     close_parsing_block:
     parse_if:
     parse_while:
     parse_false_condition_block:
     parse_true_condition_block:
     parse_else:
     parse_assert:
     skip_true_condition_else:
     parse_pure_else:
     skip_parsing:
     skip_parsing_line:
     skip_parsing_label:
     skip_parsing_instruction:
     skip_parsing_contents:
     skip_parsing_symbol:
     skip_parsing_string:
     skip_parsing_block:
     skip_parsing_end_directive:
     skip_parsing_end_block:
     close_skip_parsing_block:
     skip_parsing_else:
     parse_else_if:
     skip_parsing_pure_else:
*/
#[unsafe(no_mangle)] pub unsafe fn parser( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
parse_line_contents:
     parse_instruction_arguments:
     parse_formatter_argument:
     parse_argument:
     not_a_separator:
     foreign_argument:
     symbol_argument:
     operator_argument:
     instruction_separator:
     allow_embedded_instruction:
     embedded_instruction:
     parse_times_directive:
     parse_segment_directive:
     parse_label_directive:
     non_label_identified:
     parse_load_directive:
     parse_public_directive:
     parse_public_label:
     parse_extrn_directive:
     parse_label_operator:
     parse_from_operator:
     parse_at_operator:
     parse_quoted_extrn:
     ptr_argument:
     check_argument:
     not_instruction:
     expression_argument:
     string_argument:
     string_movsb_ok:
     string_movsw_ok:
     parse_expression:
     not_string:
     expression_comparator:
     greater:
     less:
     not_equal:
     expression:
     forced_expression:
     forced_expression_parsed:
     forced_multipart_expression:
     address_argument:
     divided_address:
     address_parsed:
     parse_address:
     unknown_segment_prefix:
     convert_address:
     forced_parenthesis:
     unallowed_character:
     open_decorator:
     close_decorator:
     close_parenthesis:
     separator:
     argument_parsed:
     expression_argument_parsed:
     contents_parsed:
     contents_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn parse_line_contents( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
identify_label:
     label_identified:
     anonymous_label_name:
     anonymous_label_name_ok:
     local_label_name:
*/
#[unsafe(no_mangle)] pub unsafe fn identify_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_operator:
     check_operator:
     next_operator:
     no_operator:
     no_simple_operator:
     operator_found:
     get_simple_operator:
     simple_operator:
     simple_next_operator:
     simple_operator_found:
*/
#[unsafe(no_mangle)] pub unsafe fn get_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_symbol:
     scan_symbols:
     symbol_ok:
     decorator_symbol:
     no_symbol:
     symbols_down:
     symbols_up:
*/
#[unsafe(no_mangle)] pub unsafe fn get_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_data_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_instruction:
     scan_instructions:
     no_instruction:
     instructions_down:
     instructions_up:
*/
#[unsafe(no_mangle)] pub unsafe fn get_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_label_id:
     composed_label_id_ok:
     anonymous_label:
     anonymous_ok:
     anonymous_back:
     bogus_anonymous:
     new_anonymous:
     new_anonymous_ok:
     standard_label:
     current_address_label:
     get_current_offset_id:
     get_counter_id:
     get_timestamp_id:
     get_org_origin_id:
     get_file_offset_id:
     current_address_label_3_characters:
     get_actual_file_offset_id:
     get_predefined_id:
     find_label:
     hash_label:
     follow_tree:
     compare_labels:
     label_found:
     extend_tree:
     add_label:
     name_first_char_ok:
     numeric_name:
     reserved_word:
     check_for_reserved_word:
     label_name_ok:
     allocate_label:
     initialize_label:
*/
#[unsafe(no_mangle)] pub unsafe fn get_label_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_expression:
     expression_loop:
     expression_element:
     expression_number:
     expression_operator:
     operators_loop:
     push_operator:
     expression_end:
     expression_converted:
     fp_expression:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_number:
     check_memory_for_number:
     valid_number:
     byte_number:
     qword_number:
     dword_number:
     word_number:
     expression_value:
     subexpression_closed:
     symbol_value:
     no_address_register:
     store_label_value:
     broken_value:
     register_value:
     preprocessor_value:
     special_preprocessor_value:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_number:
     number_begin:
     get_dec_number:
     get_dec_digit:
     next_dec_digit:
     dec_out_of_range:
     dec_out_of_range_finished:
     bad_number:
     invalid_number:
     not_number:
     get_bin_number:
     get_bin_digit:
     bin_digit_high:
     bin_out_of_range:
     bin_digit_skip:
     pascal_hex_number:
     get_hex_number:
     get_hex_digit:
     hex_letter_digit_ok:
     hex_digit_ok:
     hex_digit_high:
     hex_out_of_range:
     hex_digit_skip:
     get_oct_number:
     get_oct_digit:
     oct_digit_ok:
     oct_range_ok:
     oct_digit_wrap:
     oct_digit_high:
     oct_digit_skip:
     oct_out_of_range:
     hex_number_ok:
     pascal_hex_ok:
     number_ok:
     number_done:
     get_text_number:
     get_text_character:
     text_character_high:
     text_out_of_range:
*/
#[unsafe(no_mangle)] pub unsafe fn get_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_fp_value:
     fp_value_start:
     check_fp_value:
     digit_expected:
     fp_character_dot:
     not_fp_value:
     fp_last_character:
     fp_character_exp:
     fp_exp_sign:
     fp_character_ok:
     fp_get_sign:
     fp_get:
     fp_before_dot:
     fp_dot:
     fp_after_dot:
     fp_counter_ok:
     fp_exponent:
     fp_exponent_sign:
     get_exponent:
     exponent_ok:
     fp_power:
     fp_negative_power:
     fp_done:
     fp_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fp_mul:
    .loop:
    .done:
*/
#[unsafe(no_mangle)] pub unsafe fn fp_mul( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fp_div:
     .loop:
     .exp_ok:
     .done:
*/
#[unsafe(no_mangle)] pub unsafe fn fp_div( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fp_add:
     .exp_ok:
     .done:
     .copy:
     .change_exp:
     .exp_loop:
     .exp_done:
*/
#[unsafe(no_mangle)] pub unsafe fn fp_add( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fp_optimize:
     .loop:
     .done:
*/
#[unsafe(no_mangle)] pub unsafe fn fp_optimize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fp_zero( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn preevaluate_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
preevaluate_embedded_logical_expression:
   preevaluation_loop:
   preevaluation_done:
   preevaluate_or:
   preevaluate_and:
   leave_only_following:
   leave_only_preceding:
   quick_true:
   quick_false:
   invalid_logical_expression:
*/
#[unsafe(no_mangle)] pub unsafe fn preevaluate_embedded_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
skip_logical_value:
   negation_skipped:
   skip_logical_expression:
   logical_value_skipped:
   wrongly_structured_logical_expression:
   skip_simple_logical_value:
   find_simple_logical_value_end:
   skip_logical_value_internal_parenthesis:
   skip_logical_value_symbol:
*/
#[unsafe(no_mangle)] pub unsafe fn skip_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
preevaluate_logical_value:
   preevaluate_negation:
   preevaluate_negation_ok:
   preevaluated_expression_value:
   expression_negation_ok:
   invalid_logical_value:
   preevaluate_simple_logical_value:
   find_logical_value_boundaries:
   preevaluable_logical_operator:
   next_symbol_in_logical_value:
   logical_value_internal_parentheses:
   logical_value_boundaries_parenthesis_close:
   logical_value_boundaries_found:
   non_preevaluable_logical_value:
   leave_logical_value_intact:
   compare_symbols:
   preevaluated_false:
   store_false:
   preevaluated_true:
   store_true:
   compare_symbol_types:
   type_comparison:
   equal_type:
   types_compared:
   different_type:
   scan_symbols_list:
   get_next_from_list:
   get_from_list:
   compare_in_list:
   skip_rest_of_list:
   check_list_end:
   not_equal_in_list:
   not_equal_length_in_list:
   invalid_symbols_list:
*/
#[unsafe(no_mangle)] pub unsafe fn preevaluate_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
assembler:
     assembler_loop:
     pass_loop:
     pass_done:
     check_symbols:
     symbol_defined_ok:
     check_use_prediction:
     use_misprediction:
     use_prediction_ok:
     check_define_prediction:
     define_misprediction:
     check_next_symbol:
     symbols_checked:
     error_confirmed:
     error_handler:
     next_pass:
     assemble_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn assembler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
create_addressing_space:
     init_addressing_space:
*/
#[unsafe(no_mangle)] pub unsafe fn create_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
assemble_line:
     code_type_setting:
     new_line:
     continue_line:
     define_label:
     make_label:
     label_value_ok:
     make_virtual_label:
     finish_label:
     finish_label_symbol:
     label_symbol_ok:
     requalified_label:
     label_made:
     new_label:
     define_constant:
     make_constant:
     constant_symbol_ok:
     redeclare_constant:
     requalified_constant:
     new_constant:
     label_addressing_space:
     make_addressing_space_label:
     assemble_instruction:
     instruction_handler:
     instruction_assembled:
     line_assembled:
     source_end:
*/
#[unsafe(no_mangle)] pub unsafe fn assemble_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
org_directive:
     in_virtual:
     org_space_ok:
     org_value_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn org_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
label_directive:
     get_label_size:
     label_size_ok:
     get_free_label_value:
     make_free_label:
*/
#[unsafe(no_mangle)] pub unsafe fn label_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
load_directive:
     load_size_ok:
     value_loaded:
     get_data_point:
     get_addressing_space:
     get_data_address:
     data_address_type_ok:
     addressing_space_unavailable:
     bad_data_address:
     get_data_offset:
     data_offset_ok:
     data_offset_from_virtual:
*/
#[unsafe(no_mangle)] pub unsafe fn load_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_directive:
     sized_store:
     store_value_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn store_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
display_directive:
     display_byte:
     display_next:
*/
#[unsafe(no_mangle)] pub unsafe fn display_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
show_display_buffer:
     display_messages:
     skip_block:
     display_done:
     write_addressing_space:
     copy_output_path:
     new_path_segment:
     output_path_copied:
     append_extension:
     addressing_space_written:
*/
#[unsafe(no_mangle)] pub unsafe fn show_display_buffer( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
times_directive:
     times_argument_ok:
     times_loop:
     times_done:
     zero_times:
*/
#[unsafe(no_mangle)] pub unsafe fn times_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
virtual_directive:
     virtual_at_current:
     virtual_fallback:
     set_virtual:
     non_virtual_end_ok:
     get_extension:
     addressing_space_extension_ok:
     allocate_structure_data:
     find_structure_data:
     scan_structures:
     structure_data_found:
     no_such_structure:
     allocate_virtual_structure_data:
     continue_virtual_area:
     virtual_area_unavailable:
     end_virtual:
     remove_structure_data:
     close_virtual_addressing_space:
     virtual_byte_ok:
     virtual_word_ok:
     addressing_space_closed:
*/
#[unsafe(no_mangle)] pub unsafe fn virtual_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
repeat_directive:
    end_repeat:
    stop_repeat:
    continue_repeating:
    zero_repeat:
    find_end_repeat:
*/
#[unsafe(no_mangle)] pub unsafe fn repeat_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
while_directive:
     do_while:
     stop_while:
     while_true:
     end_while:
     find_end_while:
*/
#[unsafe(no_mangle)] pub unsafe fn while_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
if_directive:
     if_true:
     make_if_structure:
     else_true:
     else_directive:
     found_else:
     skip_else:
     end_if:
     find_else:
     else_found:
     find_end_if:
     find_structure_end:
     find_end_directive:
     skip_labels:
     labels_ok:
     structure_end:
     no_end_directive:
     skip_repeat:
     skip_while:
     skip_if:
     skip_if_block:
     skip_after_else:
     if_block_skipped:
*/
#[unsafe(no_mangle)] pub unsafe fn if_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
break_directive:
     find_breakable_structure:
     break_if:
     break_repeat:
     break_while:
*/
#[unsafe(no_mangle)] pub unsafe fn break_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
define_data:
     duplicate_data:
     duplicated_values:
     duplicate_single_data_value:
     duplicate_zero_times:
     skip_data_value:
     skip_single_data_value:
     simple_data_value:
     data_defined:
*/
#[unsafe(no_mangle)] pub unsafe fn define_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_bytes:
     get_byte:
     get_string:
     undefined_data:
     mark_undefined_data:
     undefined_data_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn data_bytes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn data_unicode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_words:
    define_words:
    get_word:
    word_data_value:
    word_string:
    copy_word_string:
    word_string_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn data_words( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_dwords:
    get_dword:
    complex_dword:
*/
#[unsafe(no_mangle)] pub unsafe fn data_dwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_pwords:
    get_pword:
    complex_pword:
*/
#[unsafe(no_mangle)] pub unsafe fn data_pwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_qwords:
    get_qword:
*/
#[unsafe(no_mangle)] pub unsafe fn data_qwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_twords:
    get_tword:
    large_shift:
    tword_mantissa_shift_done:
    store_shifted_mantissa:
    tword_exp_ok:
    fp_zero_tword:
    complex_tword:
*/
#[unsafe(no_mangle)] pub unsafe fn data_twords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
data_file:
    position_ok:
    size_ok:
    open_binary_file:
    find_current_source_path:
    get_current_path:
    cut_current_path:
    current_path_ok:
    search_in_include_paths:
    file_opened:
*/
#[unsafe(no_mangle)] pub unsafe fn data_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
reserve_bytes:
    zero_bytes:
    bytes_stosb_ok:
    bytes_stosw_ok:
    reserved_data:
*/
#[unsafe(no_mangle)] pub unsafe fn reserve_bytes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
reserve_words:
    zero_words:
    words_stosw_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn reserve_words( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
reserve_dwords:
    zero_dwords:
*/
#[unsafe(no_mangle)] pub unsafe fn reserve_dwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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

#[unsafe(no_mangle)] pub unsafe fn reserve_pwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn reserve_qwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn reserve_twords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
align_directive:
    object_alignment:
    pe_alignment:
    make_alignment:
    invalid_align_value:
    nops:
    nops_stosb_ok:
    nops_stosw_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn align_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn err_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn assert_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_expression:
    calculation_loop:
    absolute_values_calculation:
    expression_calculated:
    expression_value_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_byte_number:
    got_number:
*/
#[unsafe(no_mangle)] pub unsafe fn get_byte_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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

#[unsafe(no_mangle)] pub unsafe fn get_word_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_dword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_qword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_label:
     unadjusted_label:
     got_label:
     labeled_registers_ok:
     check_size:
     actual_file_offset_label:
     use_undefined_data_offset:
     current_file_offset_label:
     use_current_offset:
     make_file_offset_label:
     current_offset_label:
     make_current_offset_label:
     current_offset_label_ok:
     org_origin_label:
     counter_label:
     make_dword_label_value:
     timestamp_label:
     make_qword_label_value:
     predefined_label:
     label_out_of_scope:
     label_undefined:
*/
#[unsafe(no_mangle)] pub unsafe fn get_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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

#[unsafe(no_mangle)] pub unsafe fn error_undefined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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

#[unsafe(no_mangle)] pub unsafe fn force_next_pass( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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

#[unsafe(no_mangle)] pub unsafe fn undefined_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_add:
     invalid_add:
     add_relocatable:
     add_values:
     add_sign_ok:
     add_register:
     add_register_start:
     add_in_second_slot:
     create_in_first_slot:
     create_in_second_slot:
     add_register_done:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_add( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_sub:
     invalid_sub:
     negate_relocatable:
     sub_values:
     sub_sign_ok:
     sub_register:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_sub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_mul:
     swap_values:
     mul_start:
     mul_first_sign_ok:
     mul_first_register_sign_ok:
     mul_second_register_sign_ok:
     mul_second_sign_ok:
     mul_numbers:
     mul_ok:
     mul_first_register_ok:
     mul_calculated:
     mul_overflow:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_mul( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_byte_scale( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_div:
     div_first_register_ok:
     div_calculated:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_div( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_mod( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_and( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_or( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_xor:
     shr_negative:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_xor( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_shl:
     shl_over:
     shl_max:
     shl_high:
     shl_done:
     shl_overflow:
     shl_negative:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_shl( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_shr:
     shr_high:
     shr_over:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_shr( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_not:
     not_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_not( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_bsf:
     bsf_ok:
     bs_overflow:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_bsf( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_bsr:
     bsr_ok:
     finish_bs:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_bsr( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_neg:
     neg_first_register_ok:
     neg_second_register_ok:
     neg_sign_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_neg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_rva:
     rva_type_ok:
     rva_ok:
     finish_rva:
     rva_finished:
     pe64_rva:
     pe64_rva_type_ok:
     pe64_rva_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_gotoff( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_coff_rva:
     incorrect_change_of_value_type:
     change_value_type:
     calculate_elf_dyn_rva:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_coff_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_plt:
     check_value_for_plt:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_plt( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
div_64:
     divider_ok:
     div_first_sign_ok:
     div_second_sign_ok:
     div_high:
     div_high_loop:
     div_high_correction:
     div_high_done:
     div_high_large_correction:
     div_high_small_correction:
     div_done:
     remainder_ok:
     div_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn div_64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_label_reference:
     label_reference_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn store_label_reference( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn convert_fp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_fp_qword:
     fp_qword_ok:
     fp_qword_small_shift:
     fp_qword_shift_done:
     fp_qword_exp_ok:
     fp_qword_store:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_fp_qword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_fp_word:
     fp_word_ok:
     fp_word_exp_ok:
     fp_word_store:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_fp_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_fp_dword:
     fp_dword_ok:
     fp_dword_exp_ok:
     fp_dword_store:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_fp_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_string_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_byte_value:
     check_byte_value:
     byte_positive:
     return_byte_value:
     range_exceeded:
     recoverable_overflow:
     ignore_overflow:
     recoverable_misuse:
     ignore_misuse:
*/
#[unsafe(no_mangle)] pub unsafe fn get_byte_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_word_value:
     check_word_value:
     word_positive:
*/
#[unsafe(no_mangle)] pub unsafe fn get_word_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_dword_value:
     check_dword_value:
     dword_positive:
*/
#[unsafe(no_mangle)] pub unsafe fn get_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_pword_value:
     check_pword_value:
     pword_positive:
*/
#[unsafe(no_mangle)] pub unsafe fn get_pword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_qword_value:
     check_qword_value:
*/
#[unsafe(no_mangle)] pub unsafe fn get_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_count_value:
     check_count_value:
     invalid_count_value:
     zero_count:
*/
#[unsafe(no_mangle)] pub unsafe fn get_count_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_value:
     calculate_value:
     value_qword:
     truncated_value:
     value_pword:
     value_dword:
     value_word:
     value_byte:
*/
#[unsafe(no_mangle)] pub unsafe fn get_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_address_word_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_address_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn get_address_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_address_value:
     calculate_address:
     get_address_symbol_size:
     invalid_address_type:
     special_address_type_32bit:
     address_symbol_ok:
     address_sizes_mixed:
     address_sizes_mixed_type_ok:
     address_size_ok:
     check_address_registers:
     address_registers_sizes_ok:
     address_sizes_do_not_match:
     check_ip_relative_address:
     check_rip_relative_address:
     get_address_register:
     address_register_ok:
     scaled_register:
     sib_allowed:
     special_index_scale:
     check_immediate_address:
     check_index_with_base:
     check_for_ebp_base:
     swap_base_with_index:
     check_for_rbp_base:
     check_index_scale:
     check_vsib:
     check_vsib_base:
     check_vsib_base_size:
     check_vsib_index:
     swap_vsib_registers:
*/
#[unsafe(no_mangle)] pub unsafe fn get_address_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_relative_offset:
     origin_registers_ok:
     relative_offset_unallowed:
     set_relative_offset_type:
     relative_offset_ok:
     plt_relative_offset:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_relative_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
calculate_embedded_logical_expression:
     logical_loop:
     logical_or:
     logical_and:
     logical_value_already_determined:
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_embedded_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_value_for_comparison:
     first_register_size_ok:
     second_register_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_value_for_comparison( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_logical_value:
     check_for_negation:
     negation_ok:
     check_values_registers:
     values_not_relative:
     invalid_comparison:
     values_relative:
     check_less_or_greater:
     check_equal:
     check_greater:
     check_less:
     check_not_less:
     check_not_greater:
     check_not_equal:
     logical_number:
     invalid_logical_number:
     logical_number_ok:
     check_for_earlier_defined:
     check_for_defined:
     check_if_expression_defined:
     check_expression:
     defined_register:
     defined_fp_value:
     defined_string:
     check_if_symbol_defined:
     no_prediction:
     symbol_predicted_undefined:
     symbol_undefined:
     expression_checked:
     check_for_used:
     not_used:
     given_false:
     return_false:
     given_true:
     return_true:
     logical_expression:
     logical_value_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
skip_symbol:
     skip_done:
     skip_label:
     skip_instruction:
     skip_assembler_symbol:
     skip_special_label:
     skip_address:
     skip_expression:
     skip_label_value:
     skip_register:
     skip_fp_value:
     skip_string:
     nothing_to_skip:
*/
#[unsafe(no_mangle)] pub unsafe fn skip_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
expand_path:
     environment_variable:
     find_variable_end:
     not_environment_variable:
*/
#[unsafe(no_mangle)] pub unsafe fn expand_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_include_directory:
     include_directory_ok:
     path_separator_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_include_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn calculate_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_only64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_16bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
simple_instruction_16bit:
     size_prefix:
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_32bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_32bit:( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_extended_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_extended_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_extended_instruction_f3( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn iret_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn prefix_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
prefix_instruction:
    segment_prefix:
*/
#[unsafe(no_mangle)] pub unsafe fn prefix_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn bnd_prefix_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
int_instruction:
     int_imm_ok:
     take_byte_value:
*/
#[unsafe(no_mangle)] pub unsafe fn int_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
aa_instruction:
     aa_store:
*/
#[unsafe(no_mangle)] pub unsafe fn aa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
basic_instruction:
     basic_mem:
     basic_mem_reg:
     instruction_ready:
     basic_mem_imm:
     basic_mem_imm_64bit:
     basic_mem_imm_nosize:
     basic_mem_imm_8bit:
     basic_mem_imm_16bit:
     basic_mem_imm_16bit_store:
     basic_mem_simm_8bit:
     basic_mem_imm_32bit:
     basic_mem_imm_32bit_ok:
     basic_mem_imm_32bit_store:
     get_simm32:
     simm32_range_ok:
     get_simm32_ok:
     basic_reg:
     basic_reg_mem:
     basic_reg_mem_8bit:
     basic_reg_reg:
     nomem_instruction_ready:
     basic_reg_imm:
     basic_reg_imm_64bit:
     basic_reg_imm_8bit:
     basic_al_imm:
     basic_reg_imm_16bit:
     basic_reg_imm_16bit_store:
     basic_store_imm_16bit:
     basic_reg_simm_8bit:
     basic_ax_imm:
     basic_reg_imm_32bit:
     basic_reg_imm_32bit_ok:
     basic_reg_imm_32bit_store:
     basic_store_imm_32bit:
     basic_eax_imm:
     recoverable_unknown_size:
     ignore_unknown_size:
*/
#[unsafe(no_mangle)] pub unsafe fn basic_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
single_operand_instruction:
     single_mem:
     single_mem_nosize:
     single_mem_8bit:
     single_reg:
     single_reg_8bit:
*/
#[unsafe(no_mangle)] pub unsafe fn single_operand_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mov_instruction:
     mov_mem:
     mov_mem_reg:
     mov_mem_general_reg:
     mov_mem_reg_8bit:
     mov_mem_al:
     mov_mem_address32_al:
     store_mov_address32:
     mov_mem_address16_al:
     store_mov_address16:
     check_mov_address64:
     address64_required:
     address64_simm32:
     no_address64:
     mov_mem_address64_al:
     store_mov_address64:
     mov_mem_ax:
     mov_mem_address32_ax:
     mov_mem_address16_ax:
     mov_mem_address64_ax:
     mov_mem_sreg:
     mov_mem_sreg_store:
     mov_mem_imm:
     mov_mem_imm_64bit:
     mov_mem_imm_nosize:
     mov_mem_imm_8bit:
     mov_mem_imm_16bit:
     mov_mem_imm_32bit:
     mov_mem_imm_32bit_store:
     mov_reg:
     mov_reg_reg:
     mov_reg_reg_8bit:
     mov_reg_sreg:
     mov_reg_sreg64:
     mov_reg_sreg32:
     mov_reg_sreg_store:
     mov_reg_creg:
     mov_reg_creg_store:
     mov_reg_creg_64bit:
     mov_reg_mem:
     mov_reg_mem_8bit:
     mov_al_mem:
     mov_al_mem_address32:
     mov_al_mem_address16:
     mov_al_mem_address64:
     mov_ax_mem:
     mov_ax_mem_address32:
     mov_ax_mem_address16:
     mov_ax_mem_address64:
     mov_reg_imm:
     mov_reg_imm_64bit:
     mov_reg_imm_64bit_store:
     mov_reg_imm_8bit:
     mov_reg_imm_16bit:
     mov_reg_imm_32bit:
     mov_store_imm_32bit:
     store_mov_reg_imm_code:
     mov_reg_imm_prefix_ok:
     mov_reg_64bit_imm_32bit:
     mov_sreg:
     mov_sreg_reg:
     mov_sreg_reg_size_ok:
     mov_sreg_mem:
     mov_sreg_mem_size_ok:
     mov_creg:
     mov_creg_store:
     mov_creg_64bit:
*/
#[unsafe(no_mangle)] pub unsafe fn mov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
test_instruction:
     test_mem:
     test_mem_reg:
     test_mem_reg_8bit:
     test_mem_imm:
     test_mem_imm_64bit:
     test_mem_imm_nosize:
     test_mem_imm_8bit:
     test_mem_imm_16bit:
     test_mem_imm_32bit:
     test_mem_imm_32bit_store:
     test_reg:
     test_reg_reg:
     test_reg_reg_8bit:
     test_reg_imm:
     test_reg_imm_64bit:
     test_reg_imm_8bit:
     test_al_imm:
     test_reg_imm_16bit:
     test_ax_imm:
     test_reg_imm_32bit:
     test_reg_imm_32bit_store:
     test_eax_imm:
     test_reg_mem:
     test_reg_mem_8bit:
*/
#[unsafe(no_mangle)] pub unsafe fn test_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xchg_instruction:
     xchg_mem:
     xchg_reg:
     xchg_reg_reg:
     xchg_ax_reg:
     xchg_ax_reg_ok:
     xchg_ax_reg_store:
     xchg_reg_reg_store:
     xchg_reg_reg_8bit:
*/
#[unsafe(no_mangle)] pub unsafe fn xchg_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
push_instruction:
     push_next:
     push_mem:
     push_mem_16bit:
     push_mem_32bit:
     push_mem_64bit:
     push_mem_store:
     push_reg:
     push_reg_ok:
     push_reg_64bit:
     push_reg_32bit:
     push_reg_16bit:
     push_reg_store:
     push_sreg:
     push_sreg16:
     push_sreg32:
     push_sreg64:
     push_sreg_store:
     push_sreg_386:
     push_imm:
     push_imm_size_ok:
     push_imm_optimized_64bit:
     push_imm_optimized_32bit:
     push_imm_optimized_16bit:
     push_imm_8bit:
     push_imm_16bit:
     push_imm_16bit_store:
     push_imm_64bit:
     push_imm_32bit:
     push_imm_32bit_store:
     push_done:
*/
#[unsafe(no_mangle)] pub unsafe fn push_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pop_instruction:
     pop_next:
     pop_mem:
     pop_mem_16bit:
     pop_mem_32bit:
     pop_mem_64bit:
     pop_mem_store:
     pop_reg:
     pop_reg_ok:
     pop_reg_64bit:
     pop_reg_32bit:
     pop_reg_16bit:
     pop_reg_store:
     pop_done:
     pop_sreg:
     pop_sreg16:
     pop_sreg32:
     pop_sreg64:
     pop_sreg_store:
     pop_cs:
     pop_cs_store:
     pop_sreg_386:
*/
#[unsafe(no_mangle)] pub unsafe fn pop_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
inc_instruction:
     inc_mem:
     inc_mem_nosize:
     inc_mem_8bit:
     inc_reg:
     inc_reg_long_form:
     inc_reg_8bit:
*/
#[unsafe(no_mangle)] pub unsafe fn inc_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
set_instruction:
     set_mem:
     set_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn set_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
arpl_instruction:
     arpl_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn arpl_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bound_instruction:
     bound_store:
*/
#[unsafe(no_mangle)] pub unsafe fn bound_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
enter_instruction:
     enter_imm16_size_ok:
     enter_imm16_ok:
     enter_imm8_size_ok:
     enter_imm8_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn enter_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ret_instruction_only64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ret_instruction_32bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ret_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ret_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ret_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
ret_instruction:
     ret_common:
     ret_imm:
     ret_imm_ok:
     ret_imm_store:
     simple_ret:
*/
#[unsafe(no_mangle)] pub unsafe fn ret_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn retf_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn retf_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn retf_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn retf_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn lea_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
ls_instruction:
     les_instruction:
     lds_instruction:
     ls_short_code:
     ls_code_ok:
     ls_16bit:
     ls_32bit:
     ls_64bit:
*/
#[unsafe(no_mangle)] pub unsafe fn ls_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
sh_instruction:
     sh_mem:
     sh_mem_reg:
     sh_mem_cl_nosize:
     sh_mem_cl_8bit:
     sh_mem_imm:
     sh_mem_imm_size_ok:
     sh_mem_1:
     sh_mem_imm_nosize:
     sh_mem_imm_8bit:
     sh_mem_1_8bit:
     sh_reg:
     sh_reg_reg:
     sh_reg_cl_8bit:
     sh_reg_imm:
     sh_reg_imm_size_ok:
     sh_reg_1:
     sh_reg_imm_8bit:
     sh_reg_1_8bit:
*/
#[unsafe(no_mangle)] pub unsafe fn sh_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
shd_instruction:
     shd_mem:
     shd_mem_reg_imm:
     shd_mem_reg_imm_size_ok:
     shd_reg:
     shd_reg_reg_imm:
     shd_reg_reg_imm_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn shd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movx_instruction:
     movx_mem_store:
     movx_unknown_size:
     movx_reg:
     movx_reg_8bit:
     movx_reg_16bit:
*/
#[unsafe(no_mangle)] pub unsafe fn movx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movsxd_instruction:
     movsxd_mem_store:
     movsxd_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn movsxd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bt_instruction:
     bt_mem_reg:
     bt_mem_imm:
     bt_mem_imm_size_ok:
     bt_mem_imm_store:
     bt_mem_imm_nosize:
     bt_reg:
     bt_reg_reg:
     bt_reg_imm:
     bt_reg_imm_size_ok:
     bt_reg_imm_store:
*/
#[unsafe(no_mangle)] pub unsafe fn bt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bs_instruction:
     bs_reg_reg:
     get_reg_mem:
     get_reg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn bs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
ud_instruction:
     ud_reg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn ud_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
imul_instruction:
     imul_mem:
     imul_mem_nosize:
     imul_mem_8bit:
     imul_reg:
     imul_reg_8bit:
     imul_reg_:
     imul_reg_noimm:
     imul_reg_mem:
     imul_reg_mem_imm:
     imul_reg_mem_imm_64bit:
     imul_reg_mem_imm_16bit:
     imul_reg_mem_imm_16bit_store:
     imul_reg_mem_imm_32bit:
     imul_reg_mem_imm_32bit_ok:
     imul_reg_mem_imm_32bit_store:
     imul_reg_mem_imm_8bit_store:
     imul_reg_imm:
     imul_reg_reg:
     imul_reg_reg_imm:
     imul_reg_reg_imm_64bit:
     imul_reg_reg_imm_16bit:
     imul_reg_reg_imm_16bit_store:
     imul_reg_reg_imm_32bit:
     imul_reg_reg_imm_32bit_ok:
     imul_reg_reg_imm_32bit_store:
     imul_reg_reg_imm_8bit_store:
*/
#[unsafe(no_mangle)] pub unsafe fn imul_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
in_instruction:
     in_reg:
     in_ax_dx:
     in_al_dx:
     in_imm:
     in_imm_size_ok:
     in_ax_imm:
     in_al_imm:
*/
#[unsafe(no_mangle)] pub unsafe fn in_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
out_instruction:
     out_dx_ax:
     out_dx_al:
     out_imm:
     out_imm_size_ok:
     out_imm_ax:
     out_imm_al:
*/
#[unsafe(no_mangle)] pub unsafe fn out_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn call_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
jmp_instruction:
     process_jmp:
     jmp_type_ok:
     jmp_mem:
     jmp_mem_size_not_specified:
     jmp_mem_near:
     jmp_mem_64bit:
     jmp_mem_far:
     jmp_mem_48bit:
     jmp_mem_far_store:
     jmp_mem_80bit:
     jmp_mem_far_32bit:
     jmp_mem_32bit:
     jmp_mem_near_32bit:
     jmp_mem_16bit:
     jmp_reg:
     jmp_reg_64bit:
     jmp_reg_32bit:
     jmp_reg_16bit:
     jmp_imm:
     jmp_near:
     jmp_imm_32bit:
     jmp_imm_32bit_prefix_ok:
     jmp_imm_32bit_store:
     jmp_imm_32bit_ok:
     jmp_imm_64bit:
     jmp_short:
     jmp_imm_16bit:
     jmp_imm_16bit_prefix_ok:
     calculate_jump_offset:
     check_for_short_jump:
     no_short_jump:
     forced_short:
     jmp_short_value_type_ok:
     short_jump:
     jump_out_of_range:
     jmp_far:
     jmp_far_16bit:
     jmp_far_segment:
     jmp_far_32bit:
*/
#[unsafe(no_mangle)] pub unsafe fn jmp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
conditional_jump:
     conditional_jump_32bit:
     conditional_jump_32bit_prefix_ok:
     conditional_jump_32bit_store:
     conditional_jump_32bit_range_ok:
     conditional_jump_64bit:
     conditional_jump_short:
     conditional_jump_16bit:
     conditional_jump_16bit_prefix_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn conditional_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn loop_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn loop_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn loop_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
loop_instruction:
     loop_jump_32bit:
     loop_jump_32bit_prefix_ok:
     make_loop_jump:
     loop_counter_size:
     loop_counter_size_ok:
     loop_jump_64bit:
     loop_jump_16bit:
     loop_jump_16bit_prefix_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn loop_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movs_instruction:
     movs_address_32bit:
     movs_address_16bit:
     movs_store:
     movs_check_size:
*/
#[unsafe(no_mangle)] pub unsafe fn movs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
lods_instruction:
     lods_address_32bit:
     lods_address_16bit:
     lods_store:
*/
#[unsafe(no_mangle)] pub unsafe fn lods_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
stos_instruction:
     stos_address_32bit:
     stos_address_16bit:
     stos_store:
*/
#[unsafe(no_mangle)] pub unsafe fn stos_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
stos_instruction:
     stos_address_32bit:
     stos_address_16bit:
     stos_store:
*/
#[unsafe(no_mangle)] pub unsafe fn stos_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
ins_instruction:
     ins_address_32bit:
     ins_address_16bit:
     ins_store:
     ins_check_size:
*/
#[unsafe(no_mangle)] pub unsafe fn ins_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
outs_instruction:
     outs_address_32bit:
     outs_address_16bit:
     outs_store:
*/
#[unsafe(no_mangle)] pub unsafe fn outs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xlat_instruction:
     xlat_address_32bit:
     xlat_address_16bit:
     xlat_store:
*/
#[unsafe(no_mangle)] pub unsafe fn xlat_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pm_word_instruction:
     pm_mem:
     pm_mem_store:
     pm_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn pm_word_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pm_store_word_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
lgdt_instruction:
     lgdt_mem_80bit:
     lgdt_mem_48bit:
     lgdt_mem_store:
*/
#[unsafe(no_mangle)] pub unsafe fn lgdt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
lar_instruction:
     lar_reg_mem:
     lar_reg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn lar_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn invlpg_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_f2_0f_01( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_f3_0f_01_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_f3_0f_01( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn swapgs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_instruction_0f_01( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
basic_486_instruction:
     basic_486_mem_reg_8bit:
     basic_486_reg:
     basic_486_reg_reg_8bit:
*/
#[unsafe(no_mangle)] pub unsafe fn basic_486_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bswap_instruction:
     bswap_reg_code_ok:
     bswap_reg64:
*/
#[unsafe(no_mangle)] pub unsafe fn bswap_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
cmpxchgx_instruction:
     cmpxchgx_size_ok:
     cmpxchgx_store:
*/
#[unsafe(no_mangle)] pub unsafe fn cmpxchgx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
nop_instruction:
     extended_nop:
     extended_nop_store:
     extended_nop_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn nop_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
basic_fpu_instruction:
     basic_fpu_mem:
     basic_fpu_mem_32bit:
     basic_fpu_mem_64bit:
     basic_fpu_streg:
     basic_fpu_streg_st0:
     basic_fpu_st0:
     basic_fpu_single_streg:
*/
#[unsafe(no_mangle)] pub unsafe fn basic_fpu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn simple_fpu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fi_instruction:
     fi_mem_32bit:
     fi_mem_16bit:
*/
#[unsafe(no_mangle)] pub unsafe fn fi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fld_instruction:
     fld_mem_32bit:
     fld_mem_64bit:
     fld_mem_80bit:
     fld_mem_80bit_store:
     fld_streg:
     fst_streg:
*/
#[unsafe(no_mangle)] pub unsafe fn fld_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fild_instruction:
     fild_mem_32bit:
     fild_mem_16bit:
     fild_mem_64bit:
     fild_mem_64bit_store:
     fisttp_64bit_store:
*/
#[unsafe(no_mangle)] pub unsafe fn fild_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fbld_instruction:
     fbld_mem_80bit:
*/
#[unsafe(no_mangle)] pub unsafe fn fbld_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
faddp_instruction:
     faddp_streg:
*/
#[unsafe(no_mangle)] pub unsafe fn faddp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fcompp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fucompp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fxch_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ffreep_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
ffree_instruction:
     fpu_single_operand:
     fpu_streg:
*/
#[unsafe(no_mangle)] pub unsafe fn ffree_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fstenv_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fldenv_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fstenv_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fldenv_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fstenv_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fldenv_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fsave_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fnsave_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fsave_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fnsave_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fsave_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fnsave_instruction:
     fpu_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn fnsave_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fstcw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fldcw_instruction:
     fldcw_mem_16bit:
*/
#[unsafe(no_mangle)] pub unsafe fn fldcw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fstsw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fnstsw_instruction:
     fstsw_mem_16bit:
     fstsw_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn fnstsw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn finit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fninit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fcmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fcomi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fcomip_instruction:
     fcomi_streg:
     fcomi_st0_streg:
*/
#[unsafe(no_mangle)] pub unsafe fn fcomip_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
basic_mmx_instruction:
     mmx_instruction:
     mmx_mmreg_mem:
     mmx_mmreg_mmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn basic_mmx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mmx_bit_shift_instruction:
     mmx_ps_mmreg_imm8:
*/
#[unsafe(no_mangle)] pub unsafe fn mmx_bit_shift_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pmovmskb_instruction:
     pmovmskb_reg_size_ok:
     mmx_imm8:
     mmx_nomem_imm8:
     append_imm8:
*/
#[unsafe(no_mangle)] pub unsafe fn pmovmskb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pinsrw_instruction:
     pinsrw_mmreg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn pinsrw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pshufw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pshufd_instruction:
     pshuf_instruction:
     pshuf_mmreg_mmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn pshufd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movd_instruction:
     movd_reg:
     movd_mmreg:
     movd_mmreg_reg:
     get_mmx_source_register:
     make_mmx_prefix:
     no_mmx_prefix:
*/
#[unsafe(no_mangle)] pub unsafe fn movd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movq_instruction:
     movq_mem_ready:
     movq_reg:
     movq_mmreg:
     movq_mmreg_:
     movq_mmreg_reg:
     movq_mmreg_reg_store:
     movq_mmreg_mmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn movq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movdq_instruction:
     movdq_mmreg:
     movdq_mmreg_mmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn movdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn lddqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movdq2q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movq2dq_instruction:
     movq2dq_:
*/
#[unsafe(no_mangle)] pub unsafe fn movq2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse_ps_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse_pd_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cmp_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cmp_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cmpsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn comiss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn comisd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtdq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtps2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtpd2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movshdup_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
sse_instruction:
     sse_xmmreg:
     sse_reg:
     sse_reg_mem:
     sse_mem_size_ok:
     sse_ok:
     sse_cmp_mem_ok:
     sse_xmmreg_xmmreg:
     sse_xmmreg_xmmreg_ok:
     sse_nomem_ok:
     sse_cmp_nomem_ok:
     take_additional_xmm0:
     additional_xmm0_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn sse_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pslldq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movsd_instruction:
     sse_movs:
*/
#[unsafe(no_mangle)] pub unsafe fn movsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
sse_mov_instruction:
     sse_mem:
     sse_mem_xmmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn sse_mov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movlpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movhlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn maskmovq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
maskmovdqu_instruction:
     maskmov_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn maskmovdqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movmskpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movmskps_instruction:
     movmskps_reg_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn movmskps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtpi2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
cvtpi2ps_instruction:
     cvtpi_size_ok:
     cvtpi_xmmreg_xmmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn cvtpi2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtsi2ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
cvtsi2sd_instruction:
     cvtsi_instruction:
     cvtsi_xmmreg:
     cvtsi_size_ok:
     cvtsi_xmmreg_reg:
     cvtsi_xmmreg_reg_store:
*/
#[unsafe(no_mangle)] pub unsafe fn cvtsi2sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtps2pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
cvtpd2pi_instruction:
     cvtpd_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn cvtpd2pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cvtss2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
cvtsd2si_instruction:
     cvt2si_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn cvtsd2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ssse3_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
palignr_instruction:
     palignr_mmreg_mmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn palignr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
amd3dnow_instruction:
     amd3dnow_mmreg_mmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn amd3dnow_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_38_xmm0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_38_xmm0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_ss_instruction_66_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_sd_instruction_66_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
sse4_instruction_66_3a_imm8:
     sse4_instruction_66_3a_setup:
     sse4_instruction_3a_setup:
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pclmulqdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
extractps_instruction:
     extractps_size_ok:
         extractps_reg:
         setup_66_0f_3a:
*/
#[unsafe(no_mangle)] pub unsafe fn extractps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
insertps_instruction:
     insertps_size_ok:
     insertps_xmmreg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn insertps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pextrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pextrd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pextrw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pextrb_instruction:
     pextr_instruction:
     pextr_size_ok:
     pextr_prefix_ok:
     pextr_reg:
     pextr_invalid_size:
     pextrq_reg:
     pextr_reg_size_ok:
     pextr_reg_store:
*/
#[unsafe(no_mangle)] pub unsafe fn pextrb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pinsrb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pinsrd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pinsrq_instruction:
     pinsr_instruction:
     pinsr_xmmreg:
     pinsr_xmmreg_reg:
     pinsrq_xmmreg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn pinsrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pmovsxbw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pmovsxbd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pmovsxbq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pmovsxwd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pmovsxwq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
pmovsxdq_instruction:
     pmovsx_instruction:
     pmovsx_xmmreg_reg:
     setup_66_0f_38:
*/
#[unsafe(no_mangle)] pub unsafe fn pmovsxwq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xsaves_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xsaves_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fxsave_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fxsave_instruction:
     xsave_common:
     xsave_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn fxsave_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn clflush_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn cldemote_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn stmxcsr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
prefetch_instruction:
     prefetch_mem_8bit:
     prefetch_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn prefetch_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn amd_prefetch_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn clflushopt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pcommit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fence_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pause_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movntq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movntpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movntps_instruction:
     movnt_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn movntps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movntsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movntss_instruction:
     movnts_instruction:
     movnts_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn movntss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movdiri_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movnti_instruction:
     movnti_store:
*/
#[unsafe(no_mangle)] pub unsafe fn movnti_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
monitor_instruction:
     monitor_instruction_store:
*/
#[unsafe(no_mangle)] pub unsafe fn monitor_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
hreset_instruction:
     hreset_instruction_store:
*/
#[unsafe(no_mangle)] pub unsafe fn hreset_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pconfig_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn movntdqa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
extrq_instruction:
     extrq_xmmreg_xmmreg:
*/
#[unsafe(no_mangle)] pub unsafe fn extrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
insertq_instruction:
     insertq_with_imm:
*/
#[unsafe(no_mangle)] pub unsafe fn insertq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
crc32_instruction:
     crc32_reg_size_ok:
     crc32_reg_mem_store:
     crc32_unknown_size:
     crc32_reg_reg:
     crc32_reg_reg_store:
*/
#[unsafe(no_mangle)] pub unsafe fn crc32_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn popcnt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movbe_instruction:
     movbe_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn movbe_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
adx_instruction:
     adx_reg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn movbe_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
rdpid_instruction:
     rdpid_64bit:
*/
#[unsafe(no_mangle)] pub unsafe fn rdpid_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
ptwrite_instruction:
     ptwrite_mem:
     ptwrite_mem_64bit:
     ptwrite_mem_store:
     ptwrite_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn ptwrite_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vmclear_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vmxon_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
vmx_instruction:
     vmx_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn vmx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
vmread_instruction:
     vmread_nomem:
     vmread_check_size:
     vmread_long:
*/
#[unsafe(no_mangle)] pub unsafe fn vmread_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
vmwrite_instruction:
     vmwrite_nomem:
*/
#[unsafe(no_mangle)] pub unsafe fn vmwrite_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vmx_inv_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
simple_svm_instruction:
     simple_svm_detect_size:
     simple_svm_16bit:
     simple_svm_32bit:
     prefixed_svm_store:
     simple_svm_store:
*/
#[unsafe(no_mangle)] pub unsafe fn simple_svm_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn skinit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
clzero_instruction:
     clzero_64bit:
*/
#[unsafe(no_mangle)] pub unsafe fn clzero_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn invlpga_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn senduipi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
rdrand_instruction:
     senduipi_reg64:
*/
#[unsafe(no_mangle)] pub unsafe fn rdrand_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn rdfsbase_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xabort_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xbegin_instruction:
     xbegin_16bit:
     xbegin_32bit:
     xbegin_64bit:
     xbegin_address_ok:
     xbegin_rel32:
     xbegin_rel32_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn xbegin_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn bndcl_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bndcu_instruction:
     bndc_instruction:
     bndc_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn bndcu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bndmov_instruction:
     bndmov_reg:
     bndmov_reg_reg:
     take_bnd_register:
     convert_bnd_register:
*/
#[unsafe(no_mangle)] pub unsafe fn bndmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bndmk_instruction:
     bndmk_to_index:
     bndmk_selected_base:
     bndmk_ready:
     get_bnd_size:
     bnd_size_ok:
     get_address_component:
     address_component_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn bndmk_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bndldx_instruction:
     bnd_mib_check:
*/
#[unsafe(no_mangle)] pub unsafe fn bndldx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn bndstx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
take_bnd_mib:
     get_sib_address_components:
     mib_place_index:
     bnd_mib_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn take_bnd_mib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn tpause_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
umonitor_instruction:
     umonitor_reg32:
     umonitor_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn umonitor_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
movdir64b_instruction:
     movdir64b_reg_mem:
     movdir64b_ready:
     movdir64b_size_check:
*/
#[unsafe(no_mangle)] pub unsafe fn movdir64b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn enqcmd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn setssbsy_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn rstorssp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
clrssbsy_instruction:
     setup_clrssbsy:
*/
#[unsafe(no_mangle)] pub unsafe fn clrssbsy_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn rdsspq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn rdsspd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn incsspq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
incsspd_instruction:
     setup_incssp:
     cet_size_check:
     cet_dword:
*/
#[unsafe(no_mangle)] pub unsafe fn incsspd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn wrussq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn wrssq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn wrussd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
wrssd_instruction:
     wrss_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn wrssd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn endbr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn take_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_register:
     match_register_size:
     register_size_ok:
     high_byte_register:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn convert_fpu_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_mmx_register:
     xmm_register:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_mmx_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn convert_xmm_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_size_operator:
     size_operator_ok:
     no_size_operator:
*/
#[unsafe(no_mangle)] pub unsafe fn get_size_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_jump_operator:
     jump_operator_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_jump_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_address:
     get_address_of_required_size:
     calculate_relative_address:
     address_high_ok:
     clear_address_size:
     address_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
get_address_prefixes:
     get_address_size_prefix:
     address_size_prefix_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn get_address_prefixes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn operand_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
operand_32bit:
     size_prefix_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn operand_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn operand_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn operand_autodetect( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_segment_prefix_if_necessary:
     ss_prefix:
*/
#[unsafe(no_mangle)] pub unsafe fn store_segment_prefix_if_necessary( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_segment_prefix:
     segment_prefix_86:
     segment_prefix_386:
     segment_prefix_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn store_segment_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_classic_instruction_code:
     operand_prefix_ok:
     opcode_prefix_ok:
     rex_prefix_ok:
     store_extended_code:
     instruction_code_ok:
     store_supplemental_code:
*/
#[unsafe(no_mangle)] pub unsafe fn store_classic_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_nomem_instruction:
     nomem_reg_high_code_ok:
     nomem_reg_code_ok:
     nomem_rm_high_code_ok:
     nomem_rm_code_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn operand_store_nomem_instructionautodetect( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_instruction:
     reg_high_code_ok:
     reg_code_ok:
     address_value_out_of_range:
     address_value_ok:
     determine_16bit_address:
     address_bx_si:
     address_bx_di:
     address_bp_si:
     address_bp_di:
     address_si:
     address_di:
     address_bx:
     address_bp:
     postbyte_16bit:
     address_16bit_value:
     address_8bit_value:
     address:
     address_vsib:
     vsib_high_code_ok:
     vsib_index_ok:
     postbyte_32bit:
     postbyte_64bit:
     address_prefix_ok:
     base_code_ok:
     index_code_ok:
     base_and_index:
     scale_2:
     scale_1:
     scale_ok:
     sib_ready:
     address_value:
     sib_address_32bit_value:
     sib_address_8bit_value:
     sib_address:
     only_index_register:
     zero_index_register:
     only_base_register:
     simple_address_32bit_value:
     simple_address_8bit_value:
     simple_address:
     address_immediate:
     address_immediate_32bit:
     store_immediate_address:
     store_address_32bit_value:
     address_32bit_relocation:
     address_32bit_relocation_ok:
     store_address_64bit_value:
     address_64bit_relocation_ok:
     address_immediate_sib:
     address_immediate_sib_store:
     address_immediate_sib_32bit:
     address_immediate_sib_nosignextend:
     address_eip_based:
     address_rip_based:
     address_relative:
     address_relative_ok:
     addressing_16bit:
     address_immediate_16bit:
     address_16bit_prefix:
     address_32bit_prefix:
     instruction_prefix_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn store_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_single_source_pd_instruction_er_evex:
    avx_single_source_pd_instruction_er:
    avx_single_source_pd_instruction_sae_evex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_pd_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_pd_instruction_imm8:
    avx_pd_instruction_er:
    avx_pd_instruction_sae:
    avx_pd_instruction:
    avx_pd_instruction_38_evex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pd_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtudq2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_single_source_ps_instruction_er_evex:
    avx_single_source_ps_instruction_er:
    avx_single_source_ps_instruction_noevex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_ps_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_ps_instruction_imm8:
    avx_ps_instruction_er:
    avx_ps_instruction_sae:
    avx_ps_instruction:
    avx_ps_instruction_66_38_evex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_ps_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_sd_instruction_er:
    avx_sd_instruction_sae:
    avx_sd_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_sd_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_ss_instruction_er:
    avx_ss_instruction_sae:
    avx_ss_instruction:
    avx_ss_instruction_noevex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_ss_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_single_source_q_instruction_38_evex:
 avx_q_instruction_38_evex:
 avx_q_instruction_38:
 avx_q_instruction_38_w1_evex:
 avx_q_instruction_38_w1:
 avx_q_instruction_3a_imm8_w1:
 avx_q_instruction_3a_imm8_evex:
 avx_q_instruction_3a_imm8:
 avx_q_instruction_evex:
 avx_q_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_q_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_single_source_d_instruction_38_evex_w1:
    avx_single_source_d_instruction_38_evex:
    avx_single_source_d_instruction_38:
    avx_d_instruction_38_evex:
    avx_d_instruction_38:
    avx_d_instruction_3a_imm8_evex:
    avx_single_source_d_instruction_imm8:
    avx_d_instruction_evex:
    avx_d_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_d_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_bw_instruction_3a_imm8_w1_evex:
    avx_bw_instruction_3a_imm8_evex:
    avx_single_source_bw_instruction_38:
    avx_bw_instruction_38:
    avx_bw_instruction:
    avx_pi_instruction:
    avx_bw_instruction_38_w1_evex:
    avx_bw_instruction_38_evex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_bw_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_instruction:
     avx_instruction_with_broadcast:
     avx_xop_common:
     avx_reg:
     avx_vex_reg:
     avx_vex_reg_ok:
     avx_regs_size_ok:
     avx_regs_rm:
     avx_regs_mem_reg_store:
     avx_regs_reg:
     avx_regs_reg_:
     avx_regs_reg_reg:
     take_avx_rm:
     avx_reg_ok:
     take_avx_mem:
     avx_mem_broadcast_check:
     avx_mem_broadcast_ok:
     avx_mem_ok:
     avx_mem_size_ok:
     avx_mem_size_deciding:
     avx_mem_size_enforced:
     take_imm4_if_needed:
     imm4_ok:
     take_avx512_mask:
     avx512_masking_ok:
     take_avx512_rounding:
     avx512_rounding_allowed:
     take_sae:
     avx512_rounding_done:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movdqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movdqa_instruction:
     avx_movdq_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movdqa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_movdqu16_instruction:
    avx512_movdqu8_instruction:
    avx512_movdqu64_instruction:
    avx512_movdqu32_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_movdqu_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_movdqa64_instruction:
    avx512_movdqa32_instruction:
        avx_movdq_instruction_evex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_movdqa_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movps_instruction:
    avx_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movntpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movntdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movntps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_compress_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_compress_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_lddqu_instruction:
     avx_load_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_lddqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movntdqa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movd_instruction:
     avx_mov_instruction:
     avx_movd_reg:
     avx_movd_reg_ready:
     avx_movd_xmmreg:
     avx_movd_xmmreg_mem_ready:
     avx_movd_xmmreg_reg:
     avx_movq_xmmreg_xmmreg:
     avx_movq_xmmreg_xmmreg_opcode:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movd_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movq_xmmreg_xmmreg:
     avx_movq_xmmreg_xmmreg_opcode:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movq_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movddup_instruction:
     avx_movddup_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movddup_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movlpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movlps_instruction:
     avx_movlps_mem:
     avx_movlps_mem_:
     avx_movlps_mem_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movhlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movss_instruction:
     avx_movs_instruction:
     avx_movs_reg_mem:
     avx_movs_reg_mem_ok:
     avx_movs_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_comiss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_comisd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movshdup_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtqq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pshuf_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_128bit_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_128bit_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_pi_instruction_38_noevex:
     avx_instruction_38_noevex:
     avx_instruction_38:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_ss_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_sd_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_128bit_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_128bit_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_triple_source_instruction_3a_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_single_source_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_pi_instruction_3a_imm8_noevex:
     avx_instruction_3a_imm8_noevex:
     avx_instruction_3a_noevex:
     avx_instruction_3a:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pclmulqdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_instruction_38_nomask( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_pd_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pd_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_ps_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ps_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_ss_instruction_sae_imm8:
     avx512_instruction_sae_imm8:
     avx512_instruction_imm8:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_pd_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ps_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_ps_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_ss_instruction:
     avx512_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_exp2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_exp2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fma_instruction_pd( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fma_instruction_ps( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fma_instruction_sd( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fma_instruction_ss:
     fma_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn fma_instruction_ss( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fma4_instruction_p( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn fma4_instruction_sd( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
fma4_instruction_ss:
     fma4_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn fma4_instruction_ss( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmp_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmp_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmp_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_cmp_ss_instruction:
    avx_cmp_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmp_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmpeqq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmpeqd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cmpeqb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_uq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_ud_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_uw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_ub_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_cmp_b_instruction:
     avx_cmp_pi_instruction_evex:
     avx_cmp_pi_instruction:
     avx_cmp_common:
     avx_maskreg:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_fpclasspd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_fpclassps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_fpclasssd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_fpclassss_instruction:
    avx_fpclass_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_fpclassss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_ptestnmb_instruction:
     avx512_ptestnm_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_ptestmb_instruction:
     avx512_ptestm_instruction:
     avx512_ptest_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_shift_instruction_q( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_shift_instruction_d( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_b( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_d( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_q( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_w( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_b( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_d( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_q( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction_w( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mask_instruction:
     mask_instruction_nds_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn mask_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn take_mask_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn convert_mask_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
kmov_instruction:
     kmov_with_mem:
     kmov_mem_size_ok:
     setup_kmov_prefix:
     kmov_w_ok:
     kmov_prefix_ok:
     kmov_maskreg:
     kmov_maskreg_maskreg:
     kmov_maskreg_reg:
     kmov_with_reg:
     kmov_reg_size_check:
     kmov_f2_w1:
     kmov_f2:
     kmov_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn kmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_m2_instruction_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_m2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_2m_instruction_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_pmov_2m_instruction:
     setup_f3_0f_38:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_2m_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vzeroall_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vzeroupper_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vstmxcsr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_perm2f128_instruction:
     avx_instruction_imm8_without_128bit:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_perm2f128_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_shuf_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_shuf_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_permd_instruction:
     avx_instruction_without_128bit:
     setup_avx_66_supplemental:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_permd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_permq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_permilpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_permilps_instruction:
    avx_permil_instruction:
    avx_permil_size_ok:
    avx_permil_rm_or_imm8:
    avx_permq_rm:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_permilps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vpermil_2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vpermil_2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
permil2_instruction:
     vpermil2_instruction_setup:
*/
#[unsafe(no_mangle)] pub unsafe fn permil2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_shift_q_instruction_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_shift_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_shift_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_shift_bw_instruction:
     avx_shift_instruction:
     convert_avx_shift_opcode:
     avx_shift_reg_reg_reg:
     avx_shift_reg_reg_mem:
     avx_shift_reg_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_shift_bw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_shift_dq_instruction:
     avx_shift_dq_reg_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_shift_dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_rotate_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_rotate_d_instruction:
     avx512_rotate_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_rotate_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pmovsxbq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pmovsxbd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
vx_pmovsxbw_instruction:
     avx_pmovsx_instruction:
     avx_pmovsx_reg_reg:
     avx_pmovsx_xmmreg_reg_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn vx_pmovsxbw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmovqb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmovdb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_pmovwb_instruction:
     avx512_pmov_instruction:
     avx512_pmov_common:
     avx512_pmov_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_pmovwb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_128_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_32x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_32x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_32x8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_64x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_broadcast_64x4_instruction:
     avx_broadcast_instruction_w1_evex:
     avx_broadcast_instruction_evex:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_64x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_broadcastss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_broadcastsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_pbroadcastq_instruction:
     avx_broadcast_pi_instruction:
     avx_broadcast_instruction:
     avx_broadcast_destination_size_ok:
     avx_broadcast_reg_reg:
     avx_broadcast_reg_avx_reg:
     avx_broadcast_reg_avx_reg_size_ok:
     avx_broadcast_reg_general_reg:
     avx_broadcast_reg_general_reg_size_ok:
     avx_broadcast_reg_general_reg_ready:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_extract_64x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_extract_32x8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_extract_64x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_extract_32x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_extractf128_instruction:
     avx_extractf_instruction:
     avx_extractf_mem_size_ok:
     avx_extractf_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_extractf128_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_insert_64x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_insert_32x8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_insert_64x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_insert_32x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_insertf128_instruction:
     avx_insertf_instruction:
     avx_insertf_reg_reg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_insertf128_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_extract_b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_extract_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_extract_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_extract_d_instruction:
     avx_extract_instruction:
     avx_extractps_reg:
     avx_extractps_reg_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_extract_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_insertps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pinsrb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pinsrw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pinsrd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_pinsrq_instruction:
     avx_pinsr_instruction_3a:
     avx_pinsr_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pinsrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtudq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtdq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2qq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttps2qq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_cvtps2pd_instruction:
     avx_cvt_d_instruction:
     avx_cvt_d_reg_reg_size_ok:
     avx_cvt_d_reg_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtpd2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtuqq2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtpd2udq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttpd2udq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtpd2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_cvttpd2dq_instruction:
     avx_cvt_q_instruction:
     avx_cvt_q_reg_mem:
     avx_cvt_q_check_size:
     avx_cvt_q_size_ok:
     avx_cvt_q_size_not_specified:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttpd2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttps2udq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttps2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtph2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_cvtps2ph_instruction:
     vcvtps2ph_reg:
     vcvtps2ph_reg_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2ph_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtsd2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttsd2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtsd2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttsd2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtss2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttss2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtss2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_cvttss2si_instruction:
     avx_cvt_2si_instruction:
     avx_cvt_2si_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvttss2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtusi2sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi2sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtusi2ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_cvtsi2ss_instruction:
     avx_cvtsi_instruction:
     avx_cvtsi_rounding:
     avx_cvtsi_reg_reg_reg32:
     avx_cvtsi_reg_reg_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi2ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_maskmov_w1_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_maskmov_instruction:
     avx_maskmov_mem:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_maskmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movmskpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_movmskps_instruction:
     avx_movmskps_reg_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_movmskps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn avx_maskmovdqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx_pmovmskb_instruction:
     avx_pmovmskb_reg_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx_pmovmskb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn gather_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
gather_ps_instruction:
     gather_mem_size_check:
     gather_elements_size_ok:
     gather_mem_size_ok:
     gather_reg_mem_reg:
     gather_arguments_ok:
     gather_regular:
     gather_double:
     gather_uniform:
     gather_vr128:
*/
#[unsafe(no_mangle)] pub unsafe fn gather_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn scatter_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn scatter_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn gatherpf_qpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
gatherpf_dpd_instruction:
     gatherpf_pd_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn gatherpf_dpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn gatherpf_qps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
gatherpf_dps_instruction:
     gatherpf_ps_instruction:
     gatherpf_instruction:
     gatherpf_mem_size_ok:
     gatherpf_check_vsib:
*/
#[unsafe(no_mangle)] pub unsafe fn gatherpf_dps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bmi_instruction:
     bmi_reg:
     bmi_reg_reg:
     operand_32or64:
     operand_32or64_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn bmi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pdep_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn pext_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn andn_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn sarx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn shrx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn shlx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bzhi_instruction:
     bzhi_reg_reg:
     get_vex_source_register:
     no_vex_source_register:
*/
#[unsafe(no_mangle)] pub unsafe fn bzhi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
bextr_instruction:
     bextr_reg_reg:
     setup_bextr_imm_opcode:
     bextr_reg_mem_imm32:
     bextr_reg_reg_imm32:
     store_nomem_instruction_with_imm32:
     get_imm32:
*/
#[unsafe(no_mangle)] pub unsafe fn bextr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
rorx_instruction:
     rorx_reg_reg:
*/
#[unsafe(no_mangle)] pub unsafe fn rorx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn tbm_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn llwpcb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
lwpins_instruction:
     lwpins_reg_mem_size_ok:
     lwpins_reg_reg:
     prepare_lwpins:
*/
#[unsafe(no_mangle)] pub unsafe fn lwpins_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xop_single_source_sd_instruction:
 xop_single_source_ss_instruction:
 xop_single_source_instruction:
     xop_instruction_9:
 xop_single_source_128bit_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn xop_single_source_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_triple_source_128bit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xop_128bit_instruction:
     xop_instruction_8:
*/
#[unsafe(no_mangle)] pub unsafe fn xop_128bit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_ub_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_ud_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_uq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xop_pcom_uw_instruction:
     xop_pcom_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn xop_pcom_uw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn vpcmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
xop_shift_instruction:
     xop_shift_reg_reg_mem:
     xop_shift_reg_reg_imm:
     xop_shift_reg_mem:
     xop_shift_reg_mem_imm:
*/
#[unsafe(no_mangle)] pub unsafe fn xop_shift_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
avx512_4vnniw_instruction:
     reg4_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn avx512_4vnniw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn take_tile_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn take_tile_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn amx_int8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn amx_bf16_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn amx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn tilezero_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
tilerelease_instruction:
     tile_instruction:
*/
#[unsafe(no_mangle)] pub unsafe fn tilerelease_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn ldtilecfg_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
tileloadd_instruction:
     tileloadd_setup:
*/
#[unsafe(no_mangle)] pub unsafe fn tileloadd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn tilestored_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn set_evex_mode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn take_avx_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
convert_avx_register:
     avx512_register_size:
     avx_register_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn convert_avx_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_vex_instruction_code:
     prepare_vex:
     get_vex_lpp_bits:
     get_vex_pp_bits:
     vex_f2:
     vex_f3:
     vex_66:
     store_vex_0f38_instruction_code:
     store_vex_0f3a_instruction_code:
     store_vex_0f_instruction_code:
     make_c4_vex:
     check_vex:
     vex_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn store_vex_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_xop_instruction_code:
     xop_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn store_xop_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
store_evex_instruction_code:
     prepare_evex:
     make_evex:
     evex_rounding:
     evex_l_ok:
     evex_zaaa_ok:
     evex_b_ok:
     store_evex_0f38_instruction_code:
     store_evex_0f3a_instruction_code:
*/
#[unsafe(no_mangle)] pub unsafe fn store_evex_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
compress_displacement:
     calculate_displacement_scale:
     displacement_not_compressed:
     displacement_compressed:
     displacement_compression_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn compress_displacement( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
formatter:
     duplicate_output_path:
     find_extension:
     extension_found:
     sys_extension:
     efi_extension:
     bin_extension:
     obj_extension:
     o_extension:
     no_extension:
     exe_extension:
     make_extension:
     adapt_case:
     adapt_ok:
     adapt_next:
     extension_specified:
     copy_extension:
     extension_ok:
     output_path_ok:
     copy_labels:
     labels_table_ok:
     common_formatter:
     calculate_code_size:
     stub_written:
     write_output:
     output_written:
     write_code:
*/
#[unsafe(no_mangle)] pub unsafe fn formatter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
format_directive:
     select_format:
     format_defined:
     format_prefix:
*/
#[unsafe(no_mangle)] pub unsafe fn format_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn entry_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn stack_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn heap_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn segment_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn section_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
public_directive:
     public_allowed:
     public_label:
*/
#[unsafe(no_mangle)] pub unsafe fn public_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
extrn_directive:
     extrn_allowed:
     get_extrn_size:
     extrn_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn extrn_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mark_relocation:
     relocation_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn mark_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn close_pass( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn format_mz( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mark_mz_relocation:
     mz_relocation_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn mark_mz_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mz_segment:
     segment_type_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn mz_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mz_entry:
     initial_cs_ok:
     recoverable_invalid_address:
     ignore_invalid_address:
*/
#[unsafe(no_mangle)] pub unsafe fn mz_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
mz_stack:
     stack_pointer:
     initial_ss_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn mz_stack( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mz_heap( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
write_mz_header:
     mz_stack_ok:
     mz_size_ok:
*/
#[unsafe(no_mangle)] pub unsafe fn write_mz_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
*/
#[unsafe(no_mangle)] pub unsafe fn mz_heap( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
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
19954 */
