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
        call	operand_32or64
        ret
        bextr_reg_mem_imm32:
        call	get_imm32
*/
#[unsafe(no_mangle)] pub unsafe fn start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn time_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_bytes_count( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_param( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_output_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn option_param( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bad_params( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn memory_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_memory_setting( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn passes_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_passes_setting( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_param( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn definition_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_definition( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbols_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_symbols_setting( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_option_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_option_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn option_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_option_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_definition_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_definition_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bad_definition_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_definition_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn definition_value_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn collect_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_path_to_low_memory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_memory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn high_brk( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn mmap_unusable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn mmap_with_hint( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_low_memory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn mmap_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_variable_names( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_environment_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_of_variable_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_variable_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn adapt_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn path_char_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn create( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn do_create( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn read( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn file_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn write( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn lseek( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn digit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_break_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn block_displayed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assembler_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_error_lines( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_definition_origin( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_next_error_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_error_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_number_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_line_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_line_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_data_displayed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn space_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn quoted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_quoted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn quote_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn quoted_copied( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instruction_converted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_error_message( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn stack_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn main_file_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_failed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_limitations_exceeded( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_definition( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn general_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn error_reading_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_file_format( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_macro_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn incomplete_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unexpected_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn illegal_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_operand( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_operand_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operand_size_not_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operand_sizes_do_not_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_address_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn address_sizes_do_not_agree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn disallowed_combination_of_registers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn long_immediate_not_encodable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn relative_jump_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn value_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_asciiz( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_quoted_symbol_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_out_of_scope( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_symbol_out_of_scope_message( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_use_of_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn name_too_long( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserved_word_used_as_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_already_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn missing_end_quote( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn missing_end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unexpected_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn extra_characters_on_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_not_aligned_enough( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn setting_already_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_already_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn too_many_repeats( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assertion_failed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invoked_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn error_with_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_strings_table( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_external_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_section_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_elf_section_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_default_section_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn strings_table_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_labels_dump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_name_outside_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_dump_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_dump_line_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_base_symbol_for_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn base_symbol_for_label_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_defined_flag_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_used_flag_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn labels_dump_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_lines_dump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_line_dump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn base_symbol_for_line_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn lines_dump_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_inexisting_offsets( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn correct_inexisting_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_references_dump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn dump_reference( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn references_dump_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn setup_dump_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_not_from_main_input( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_next_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessed_source_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_line_content( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn restore_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn restore_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessed_line_source_restored( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn restore_next_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessed_source_restored( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn dump_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_characters_table( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_symbol_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_predefinitions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_predefinition( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn found_predefinition_separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_predefinition_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_backslash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn group_predefinition_backslashes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_backslashed_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_predefinition_backslashed_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_backslashed_symbol_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinition_converted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefinitions_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_postponed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_postponed_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_postponed_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_earliest_postponed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn earliest_postponed_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessing_finished( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn use_postponed_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn file_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_line_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn found_separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn control_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn lf_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn cr_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn backslash_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn group_backslashes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_end_quote( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn backslashed_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_backslashed_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn backslashed_symbol_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_lines( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_concatenated_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_lf( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_cr( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn ignore_comment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_case( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn case_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_directives( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn directive_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn directive_handler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_directive_handler_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess_current_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_start_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_fix_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_preprocessing( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn initial_preprocessing_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_preprocessor_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_symbolic_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn struc_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbolic_constant_in_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_for_broken_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_broken( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_constant_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn move_rest_of_line_up( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn replace_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_preprocessed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessor_special_symbol_not_recognized( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_preprocessor_special_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn follow_hashes_roots( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn follow_hashes_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_with_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_equal_hash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessor_symbol_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessor_symbol_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_hash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fnv1a_hash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessor_symbol_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reshape_hash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_leave_for_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_entry_to_reuse( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_symbol_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reuse_symbol_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn extend_hashes_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_equ_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_preprocessor_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_symbolic_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_argument_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_arguments_finisher( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_macro_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_argument_with_default_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_argument_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn enclosed_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn enclosed_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn enclosed_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn enclosed_argument_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn simple_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn argument_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn argument_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn argument_value_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn found_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn postpone_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn rept_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irp_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irps_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irpv_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn match_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_instant_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parameters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_quoted_parameter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parameters_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_pattern( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_symbol_in_pattern( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_quoted_string_in_pattern( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn pattern_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn purge_struc( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn restore_equ_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn restore_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_symbol_to_restore( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_restored( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_equ_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_symbolic_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn ignore_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_brace( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_replacing( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn replace_symbolic_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_after_replaced( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn move_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn movsb_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn movsw_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_after_replaced( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn brace_after_replaced( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_after_replaced( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn replace_special_symbolic_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessed_file_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessed_line_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_current_line_from_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_line_from_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_from_file_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn before_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_more_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_before_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_before_macro_operators_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_before_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn escaped_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reduce_symbol_conversion( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_conversion( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_character_conversion( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_to_quoted_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn shift_line_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_concatenation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_concatenation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn do_symbol_concatenation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_escaped_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_concatenation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_converted_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_concatenating_converted_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn concatenate_converted_symbol_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn do_string_concatenation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn after_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_after_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_after_macro_operatorss_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_after_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_macro_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_arguments_group( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_default_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn required_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn greedy_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn got_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_argument_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn argument_value_length_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn arguments_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn use_instant_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_rept_counter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn rept_counter_added( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn rept_counters_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instant_macro_parameters_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instant_macro_finish( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instant_macro_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instant_macro_attached_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn precalculate_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn value_precalculated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn do_irp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irp_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irp_with_default_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irps_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irp_parameters_start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_irp_parameter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irp_parameters_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_irps_parameter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irps_quoted_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irps_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irps_parameter_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn irps_parameters_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_irpv_parameter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_variable_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_variable_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn variable_values_marked( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_irpv_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn collect_next_variable_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn variable_values_collected( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn free_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_exact_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn try_different_matching( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn match_more_elements( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_element( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_quoted_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn cannot_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn exact_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn exact_match_complete( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn match_verbatim( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn match_elements( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_characters_matched( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn match_quoted_strings( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn match_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_elements( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn elements_mismatch( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_matching( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn matched_pattern( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_matched_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn matched_symbols_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_macro_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_instructions_start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instant_macro_line_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_defining_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn defining_directive_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_line_header_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro_line_element( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_macro_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_macro_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn replace_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn group_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn multiple_macro_symbol_values( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn enclose_macro_symbol_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_symbol_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn multiple_macro_symbol_values_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn replace_macro_counter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn group_macro_counter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn multiple_macro_counter_values( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_number_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn numer_symbol_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_number_digits( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_number_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn number_digit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_raw_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_struc_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn disable_replaced_struc_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_foreign_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_foreign_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_foreign_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_foreign_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_line_processed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_next_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_block_processed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_local_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn counter_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn letter_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn small_letter_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn counter_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn common_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn forward_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reverse_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reverse_counter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn continue_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn block_closed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn try_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_symbol_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn macro_symbol_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_macro_symbol_leaf( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn follow_macro_symbols_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_such_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn extend_macro_symbol_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_current_file_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_current_file_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn cut_current_file_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn current_file_path_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn try_include_directories( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn try_in_current_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn include_path_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_preprocessed_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parser_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_data_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn simple_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn block_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn constant_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn main_instruction_identified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn common_parse( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn empty_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn empty_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_rest_of_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_next_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn source_parsed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn blocks_stack_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_end_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_parsing_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_false_condition_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_true_condition_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_assert( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_true_condition_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_pure_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_contents( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_end_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_skip_parsing_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_else_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_parsing_pure_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_instruction_arguments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_formatter_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_a_separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn foreign_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operator_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instruction_separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn allow_embedded_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn embedded_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_times_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_segment_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_label_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn non_label_identified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_load_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_public_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_public_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_extrn_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_label_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_from_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_at_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_quoted_extrn( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn ptr_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_movsb_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_movsw_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_comparator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn greater( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn less( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_equal( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn forced_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn forced_expression_parsed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn forced_multipart_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn address_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn divided_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn address_parsed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn parse_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unknown_segment_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn forced_parenthesis( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unallowed_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn open_decorator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_decorator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_parenthesis( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn separator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn argument_parsed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_argument_parsed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn contents_parsed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn contents_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_identified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_label_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_label_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn local_label_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_simple_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operator_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_simple_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn simple_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn simple_next_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn simple_operator_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn decorator_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbols_down( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbols_up( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_instructions( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instructions_down( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instructions_up( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn composed_label_id_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_back( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bogus_anonymous( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_anonymous( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_anonymous_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn standard_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn current_address_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_current_offset_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_counter_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_timestamp_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_org_origin_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_file_offset_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn current_address_label_3_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_actual_file_offset_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_predefined_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hash_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn follow_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_labels( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn extend_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn name_first_char_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn numeric_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserved_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_for_reserved_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn initialize_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_element( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn operators_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn push_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_converted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_memory_for_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn valid_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn byte_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn qword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn dword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn word_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn subexpression_closed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_address_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_label_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn broken_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn register_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessor_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn special_preprocessor_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn number_begin( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_dec_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_dec_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_dec_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn dec_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn dec_out_of_range_finished( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bad_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_bin_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_bin_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bin_digit_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bin_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bin_digit_skip( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn pascal_hex_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_hex_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_hex_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hex_letter_digit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hex_digit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hex_digit_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hex_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hex_digit_skip( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_oct_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_oct_digit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn oct_digit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn oct_range_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn oct_digit_wrap( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn oct_digit_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn oct_digit_skip( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn oct_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn hex_number_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn pascal_hex_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn number_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn number_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_text_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_text_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn text_character_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn text_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_value_start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn digit_expected( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_character_dot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_last_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_character_exp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_exp_sign( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_character_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_get_sign( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_get( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_before_dot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_dot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_after_dot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_counter_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_exponent( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_exponent_sign( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_exponent( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn exponent_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_power( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_negative_power( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_mul( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_mul_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_mul_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_div( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_div_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_div_exp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_div_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add_exp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add_copy( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add_change_exp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add_exp_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add_exp_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_optimize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_optimize_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_optimize_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_zero( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_embedded_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluation_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluation_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_or( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_and( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn leave_only_following( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn leave_only_preceding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn quick_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn quick_false( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn negation_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn wrongly_structured_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_simple_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_simple_logical_value_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_logical_value_internal_parenthesis( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_logical_value_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_negation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_negation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluated_expression_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_negation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_simple_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_logical_value_boundaries( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluable_logical_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_symbol_in_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_internal_parentheses( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_boundaries_parenthesis_close( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_boundaries_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn non_preevaluable_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn leave_logical_value_intact( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluated_false( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_false( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluated_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_symbol_types( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn type_comparison( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn equal_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn types_compared( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn different_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_symbols_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_next_from_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_from_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_in_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_rest_of_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_list_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_equal_in_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn not_equal_length_in_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_symbols_list( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assembler_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn pass_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn pass_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_defined_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_use_prediction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn use_misprediction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn use_prediction_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_define_prediction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_misprediction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_next_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbols_checked( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn error_confirmed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn error_handler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_pass( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assemble_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn init_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn code_type_setting( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn continue_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_virtual_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_label_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn requalified_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_made( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn constant_symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn redeclare_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn requalified_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_addressing_space_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assemble_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instruction_handler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn instruction_assembled( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn line_assembled( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn source_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn in_virtual( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn org_space_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn org_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_label_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn label_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_free_label_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_free_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn load_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn load_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn value_loaded( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_data_point( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_data_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_address_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn addressing_space_unavailable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bad_data_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_data_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_offset_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_offset_from_virtual( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn sized_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_byte( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_next( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn show_display_buffer( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_messages( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn display_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_output_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_path_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn output_path_copied( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn append_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn addressing_space_written( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn times_argument_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn times_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn times_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_times( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn virtual_at_current( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn virtual_fallback( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn set_virtual( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn non_virtual_end_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn addressing_space_extension_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_structures( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn structure_data_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_such_structure( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_virtual_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn continue_virtual_area( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn virtual_area_unavailable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_virtual( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn remove_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_virtual_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn virtual_byte_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn virtual_word_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn addressing_space_closed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn repeat_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn stop_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn continue_repeating( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn while_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn do_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn stop_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn while_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn if_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn if_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_if_structure( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn else_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn else_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn found_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn else_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_structure_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_labels( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn labels_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn structure_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_if_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_after_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn if_block_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn break_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_breakable_structure( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn break_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn break_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn break_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn duplicate_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn duplicated_values( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn duplicate_single_data_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn duplicate_zero_times( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_data_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_single_data_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn simple_data_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_bytes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_byte( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_undefined_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_data_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_unicode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_words( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn define_words( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn word_data_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn word_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_word_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn word_string_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_dwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn complex_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_pwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_pword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn complex_pword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_qwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_qword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_twords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_tword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn large_shift( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn tword_mantissa_shift_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_shifted_mantissa( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn tword_exp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn fp_zero_tword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn complex_tword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn position_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn open_binary_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_current_source_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_current_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn cut_current_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn current_path_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn search_in_include_paths( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn file_opened( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_bytes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_bytes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bytes_stosb_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn bytes_stosw_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserved_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_words( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_words( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn words_stosw_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_dwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_dwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_pwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_qwords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_twords( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn align_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn object_alignment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_alignment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_alignment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_align_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn nops( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn nops_stosb_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn nops_stosw_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn err_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn assert_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn calculation_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn absolute_values_calculation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_calculated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn expression_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_byte_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn got_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_word_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_dword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_qword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn unadjusted_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn got_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn labeled_registers_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn actual_file_offset_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn use_undefined_data_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn current_file_offset_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn use_current_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_file_offset_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )
        }
}

#[unsafe(no_mangle)] pub unsafe fn current_offset_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn make_current_offset_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn current_offset_label_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn org_origin_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn counter_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn make_dword_label_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn timestamp_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn make_qword_label_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn predefined_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn label_out_of_scope( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn label_undefined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn error_undefined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn force_next_pass( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn undefined_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_add( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invalid_add( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_relocatable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_values( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_register_start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_in_second_slot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn create_in_first_slot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn create_in_second_slot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn add_register_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_sub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invalid_sub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn negate_relocatable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sub_values( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sub_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sub_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_mul( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn swap_values( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_start( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_first_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_first_register_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_second_register_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_second_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_numbers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_first_register_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_calculated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mul_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_byte_scale( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_div( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_first_register_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_calculated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_mod( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_and( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_or( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_xor( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shr_negative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_shl( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shl_over( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shl_max( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shl_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shl_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shl_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shl_negative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_shr( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shr_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shr_over( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_not( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn not_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_bsf( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bsf_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bs_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_bsr( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bsr_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn finish_bs( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_neg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn neg_first_register_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn neg_second_register_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn neg_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rva_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rva_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn finish_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rva_finished( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pe64_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pe64_rva_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pe64_rva_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_gotoff( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_coff_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn incorrect_change_of_value_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn change_value_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_elf_dyn_rva( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_plt( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_value_for_plt( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn divider_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_first_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_second_sign_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_high( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_high_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_high_correction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_high_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_high_large_correction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_high_small_correction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn remainder_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn div_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_label_reference( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn label_reference_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_fp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_fp_qword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_qword_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_qword_small_shift( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_qword_shift_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_qword_exp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_qword_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_fp_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_word_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_word_exp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_word_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_fp_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_dword_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_dword_exp_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fp_dword_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_string_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_byte_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn byte_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn return_byte_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn range_exceeded( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ignore_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_misuse( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ignore_misuse( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_word_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_word_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn word_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn dword_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_pword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_pword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pword_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_count_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_count_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invalid_count_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn zero_count( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn value_qword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn truncated_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn value_pword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn value_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn value_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn value_byte( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_word_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_symbol_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invalid_address_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn special_address_type_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_sizes_mixed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_sizes_mixed_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_address_registers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_registers_sizes_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_sizes_do_not_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_ip_relative_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_rip_relative_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_register_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn scaled_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sib_allowed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn special_index_scale( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_immediate_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_index_with_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_ebp_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn swap_base_with_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_rbp_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_index_scale( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_vsib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_vsib_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_vsib_base_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_vsib_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn swap_vsib_registers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn origin_registers_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn relative_offset_unallowed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn set_relative_offset_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn relative_offset_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn plt_relative_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_embedded_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_or( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_and( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_already_determined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_value_for_comparison( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn first_register_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn second_register_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_negation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn negation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_values_registers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn values_not_relative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invalid_comparison( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn values_relative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_less_or_greater( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_equal( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_greater( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_less( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_not_less( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_not_greater( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_not_equal( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invalid_logical_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_number_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_earlier_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_if_expression_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn defined_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn defined_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn defined_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_if_symbol_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn no_prediction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn symbol_predicted_undefined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn symbol_undefined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn expression_checked( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_used( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn not_used( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn given_false( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn return_false( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn given_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn return_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_assembler_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_special_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_label_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skip_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nothing_to_skip( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn environment_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn find_variable_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn not_environment_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_include_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn include_directory_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn path_separator_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_only64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_16bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn size_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_32bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn iret_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_extended_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_extended_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_extended_instruction_f3( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn prefix_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn segment_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bnd_prefix_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn int_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn int_imm_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_byte_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn aa_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn instruction_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_16bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_simm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_32bit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_mem_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_simm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simm32_range_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_simm32_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nomem_instruction_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_al_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_16bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_store_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_simm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_ax_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_32bit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_reg_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_store_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_eax_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_unknown_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ignore_unknown_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn single_operand_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn single_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn single_mem_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn single_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn single_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn single_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_general_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_al( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_address32_al( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_mov_address32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_address16_al( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_mov_address16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_mov_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address64_required( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address64_simm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn no_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_address64_al( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_mov_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_ax( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_address32_ax( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_address16_ax( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_address64_ax( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_sreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_sreg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_mem_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_sreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_sreg64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_sreg32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_sreg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_creg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_creg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_creg_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_al_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_al_mem_address32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_al_mem_address16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_al_mem_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_ax_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_ax_mem_address32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_ax_mem_address16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_ax_mem_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_64bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_store_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_mov_reg_imm_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_64bit_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_sreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_sreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_sreg_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_sreg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_sreg_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_creg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_creg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mov_creg_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_mem_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_al_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_ax_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_eax_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn test_reg_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_ax_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_ax_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_ax_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_reg_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xchg_reg_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_next( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_reg_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_reg_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_reg_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_sreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_sreg16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_sreg32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_sreg64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_sreg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_sreg_386( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_optimized_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_optimized_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_optimized_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_16bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn push_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_next( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_reg_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_reg_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_reg_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_sreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_sreg16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_sreg32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_sreg64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_sreg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_cs( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_cs_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pop_sreg_386( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_mem_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_reg_long_form( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn inc_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn set_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn set_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn set_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn arpl_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn arpl_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bound_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bound_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn enter_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn enter_imm16_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn enter_imm16_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn enter_imm8_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn enter_imm8_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_only64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_32bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_common( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_imm_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ret_imm_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_ret( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn retf_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn retf_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn retf_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn retf_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lea_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ls_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn les_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lds_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ls_short_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ls_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ls_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ls_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ls_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_cl_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_cl_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_imm_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_mem_1_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_cl_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_imm_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sh_reg_1_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_mem_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_mem_reg_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_reg_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn shd_reg_reg_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movx_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movx_unknown_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movx_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movx_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movx_reg_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movsxd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movsxd_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movsxd_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_mem_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_mem_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_mem_imm_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_mem_imm_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_reg_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bt_reg_imm_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bs_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ud_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ud_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_mem_nosize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_noimm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_16bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_32bit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_mem_imm_8bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_16bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_32bit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imul_reg_reg_imm_8bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_ax_dx( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_al_dx( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_ax_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn in_al_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_dx_ax( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_dx_al( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_imm_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_imm_ax( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn out_imm_al( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn process_jmp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_size_not_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_near( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_far( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_48bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_far_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_80bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_far_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_near_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_reg_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_reg_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_reg_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_near( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_32bit_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_32bit_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_short( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_imm_16bit_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_jump_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn check_for_short_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn no_short_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn forced_short( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_short_value_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn short_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jump_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_far( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_far_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_far_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jmp_far_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_32bit_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_32bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_32bit_range_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_short( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn conditional_jump_16bit_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_jump_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_jump_32bit_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn make_loop_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_counter_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_counter_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_jump_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_jump_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn loop_jump_16bit_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movs_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movs_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movs_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movs_check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lods_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lods_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lods_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lods_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn stos_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn stos_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn stos_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn stos_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmps_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmps_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmps_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ins_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ins_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ins_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ins_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ins_check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn outs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn outs_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn outs_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn outs_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xlat_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xlat_address_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xlat_address_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xlat_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pm_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pm_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pm_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pm_store_word_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lgdt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lgdt_mem_80bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lgdt_mem_48bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lgdt_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lar_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lar_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lar_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invlpg_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_f2_0f_01( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_f3_0f_01_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_f3_0f_01( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn swapgs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_instruction_0f_01( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_486_mem_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_486_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_486_reg_reg_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bswap_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bswap_reg_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bswap_reg64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmpxchgx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmpxchgx_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmpxchgx_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nop_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extended_nop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extended_nop_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extended_nop_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_streg_st0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_st0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn basic_fpu_single_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_fpu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fi_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fi_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fld_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fld_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fld_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fld_mem_80bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fld_mem_80bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fld_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fst_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fild_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fild_mem_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fild_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fild_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fild_mem_64bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fisttp_64bit_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fbld_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fbld_mem_80bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn faddp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn faddp_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fcompp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fucompp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fxch_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ffreep_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ffree_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fpu_single_operand( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fpu_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fldenv_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fstenv_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fldenv_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fstenv_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fldenv_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fsave_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fnsave_instruction_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fsave_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fnsave_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fsave_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fnsave_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fpu_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fstcw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fldcw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fldcw_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fstsw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fnstsw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fstsw_mem_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fstsw_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn finit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fninit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fcmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fcomi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fcomip_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fcomi_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fcomi_st0_streg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_mmreg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_mmreg_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_bit_shift_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_ps_mmreg_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovmskb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovmskb_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mmx_nomem_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn append_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsrw_mmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pshufw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pshufd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pshuf_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pshuf_mmreg_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movd_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movd_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movd_mmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_mmx_source_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn make_mmx_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn no_mmx_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_mem_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_mmreg_( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_mmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_mmreg_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq_mmreg_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdq_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdq_mmreg_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn lddqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movq2dq_( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_pd_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmp_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmp_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmp_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmpsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmp_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cmp_sx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn comiss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn comisd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtdq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtps2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtpd2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movshdup_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_cmp_mem_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_xmmreg_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_xmmreg_xmmreg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_nomem_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_cmp_nomem_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_additional_xmm0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn additional_xmm0_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_movs( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_mov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse_mem_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movlpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movhlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn maskmovq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn maskmovdqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn maskmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movmskpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movmskps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movmskps_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtpi2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtpi_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtpi_xmmreg_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi2ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi2sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi_xmmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsi_xmmreg_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtps2pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtpd2pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtss2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvtsd2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cvt2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn palignr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn palignr_mmreg_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn amd3dnow_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn amd3dnow_mmreg_mmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_38_xmm0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_ss_instruction_66_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_sd_instruction_66_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_66_3a_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_3a_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sse4_instruction_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pclmulqdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extractps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extractps_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extractps_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_66_0f_3a( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn insertps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn insertps_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn insertps_xmmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextrd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextrw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextrb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_invalid_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextrq_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pextr_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsrb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsrd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsr_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsr_xmmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pinsrq_xmmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsxbw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsxbd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsxbq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsxwd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsxwq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsxdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pmovsx_xmmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_66_0f_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xsaves_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fxsave_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fxsave_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xsave_common( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xsave_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn clflush_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cldemote_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn stmxcsr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn prefetch_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn prefetch_mem_8bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn prefetch_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn amd_prefetch_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn clflushopt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pcommit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fence_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn pause_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movntq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movntpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movntps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movnt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movntss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movnts_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movnts_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movnti_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movnti_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn monitor_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn monitor_instruction_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn hreset_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn hreset_instruction_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movntdqa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn extrq_xmmreg_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn insertq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn insertq_with_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn crc32_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn crc32_reg_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn crc32_unknown_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn crc32_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn crc32_reg_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn popcnt_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movbe_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movbe_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn adx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn adx_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rdpid_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rdpid_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ptwrite_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ptwrite_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ptwrite_mem_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ptwrite_mem_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ptwrite_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmxon_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmx_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmread_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmread_nomem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmread_check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmread_long( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmwrite_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmwrite_nomem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vmx_inv_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_svm_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_svm_detect_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_svm_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_svm_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn prefixed_svm_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_svm_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn skinit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn clzero_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn clzero_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn invlpga_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rdrand_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn senduipi_reg64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rdfsbase_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_address_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_rel32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xbegin_rel32_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndcu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndc_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndc_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmov_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmov_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_bnd_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_bnd_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmk_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmk_to_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmk_selected_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndmk_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_bnd_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bnd_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_component( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_component_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndldx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bnd_mib_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bndstx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_bnd_mib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_sib_address_components( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mib_place_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn bnd_mib_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn umonitor_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn umonitor_reg32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn umonitor_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdir64b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdir64b_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdir64b_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn movdir64b_size_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn enqcmd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rstorssp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn clrssbsy_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_clrssbsy( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rdsspq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rdsspd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn incsspq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn incsspd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_incssp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cet_size_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn cet_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn wrussq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn wrssq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn wrussd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn wrssd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn wrss_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn endbr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn match_register_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn register_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn high_byte_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_fpu_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_mmx_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn xmm_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_xmm_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_size_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn size_operator_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn no_size_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_jump_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn jump_operator_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_of_required_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn calculate_relative_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_high_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn clear_address_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_prefixes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn get_address_size_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_size_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn operand_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn operand_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn size_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn operand_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn operand_autodetect( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_segment_prefix_if_necessary( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn ss_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_segment_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn segment_prefix_86( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn segment_prefix_386( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn segment_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_classic_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn operand_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn opcode_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn rex_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_extended_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn instruction_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_supplemental_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_nomem_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nomem_reg_high_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nomem_reg_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nomem_rm_high_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn nomem_rm_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn reg_high_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn reg_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_value_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn determine_16bit_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_bx_si( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_bx_di( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_bp_si( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_bp_di( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_si( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_di( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_bx( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_bp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn postbyte_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_16bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_8bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_vsib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vsib_high_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vsib_index_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn postbyte_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn postbyte_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn base_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn index_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn base_and_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn scale_2( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn scale_1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn scale_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sib_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sib_address_32bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sib_address_8bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn sib_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn only_index_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn zero_index_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn only_base_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_address_32bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_address_8bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn simple_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_immediate_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_address_32bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_32bit_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_32bit_relocation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_address_64bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_64bit_relocation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate_sib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate_sib_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate_sib_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate_sib_nosignextend( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_eip_based( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_rip_based( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_relative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_relative_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn addressing_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_16bit_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn address_32bit_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn instruction_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_pd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_pd_instruction_sae_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtudq2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_ps_instruction_er_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_ps_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_ps_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction_66_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_sd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_sd_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ss_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ss_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ss_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_q_instruction_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_38_w1_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_38_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_3a_imm8_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_3a_imm8_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_d_instruction_38_evex_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_d_instruction_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_d_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_d_instruction_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_d_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_d_instruction_3a_imm8_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_d_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_d_instruction_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction_3a_imm8_w1_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction_3a_imm8_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_bw_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction_38_w1_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_bw_instruction_38_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pd_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ps_instruction_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_with_broadcast( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_xop_common( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_vex_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_vex_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_regs_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_regs_rm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_regs_mem_reg_store( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_regs_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_regs_reg_( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_regs_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_avx_rm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_avx_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_broadcast_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_broadcast_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_size_deciding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_size_enforced( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_imm4_if_needed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn imm4_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_avx512_mask( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_masking_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_avx512_rounding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_rounding_allowed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_rounding_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movdqa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_movdqu16_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_movdqu8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_movdqu64_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_movdqu32_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_movdqa64_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_movdqa32_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movdq_instruction_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movntpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movntdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movntps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_compress_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_compress_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_lddqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_load_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movntdqa_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_mov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movd_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movd_reg_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movd_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movd_xmmreg_mem_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movd_xmmreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movq_xmmreg_xmmreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movq_xmmreg_xmmreg_opcode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movddup_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movddup_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movlpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movlps_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movlps_mem_( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movlps_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movhlps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movs_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movs_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movs_reg_mem_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movs_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_comisd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_movshdup_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtqq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pshuf_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_128bit_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_128bit_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_38_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_ss_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_sd_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_128bit_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_128bit_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_triple_source_instruction_3a_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_single_source_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_3a_imm8_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_3a_noevex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_3a( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pi_instruction_3a_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_pclmulqdq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_38_nomask( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pd_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_ps_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ps_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_instruction_sae_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_instruction_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_pd_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ps_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_ps_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_single_source_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_er( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction_sae( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_exp2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_exp2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma_instruction_ps( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma_instruction_sd( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma_instruction_ss( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma4_instruction_sd( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma4_instruction_ss( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn fma4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmpeqq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmpeqd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmpeqb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_uq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_ud_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_uw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_ub_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_cmp_b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_pi_instruction_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_cmp_common( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_maskreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_fpclasspd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_fpclassps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_fpclasssd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_fpclassss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_fpclass_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnmb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestnm_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestmb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptestm_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_ptest_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_shift_instruction_d( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_b( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_d( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_q( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_single_source_w( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_b( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_d( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_q( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_w( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn mask_instruction_nds_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn take_mask_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn convert_mask_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_with_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_kmov_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_w_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_maskreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_maskreg_maskreg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_maskreg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_with_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_reg_size_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_f2_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_f2( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn kmov_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_m2_instruction_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_m2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_2m_instruction_w1( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_2m_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_f3_0f_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vzeroupper_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vstmxcsr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_imm8_without_128bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_shuf_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx512_shuf_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_instruction_without_128bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn setup_avx_66_supplemental( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permilpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permilps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permil_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permil_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permil_rm_or_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_permq_rm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vpermil_2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vpermil_2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vpermil2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn vpermil2_instruction_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 )}
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_bw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_avx_shift_opcode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_reg_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_reg_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_shift_dq_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_rotate_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_rotate_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_rotate_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovsxbd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovsxbw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovsx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovsx_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovsx_xmmreg_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmovqb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmovdb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmovwb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_common( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_32x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_32x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_32x8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_64x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_broadcast_64x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_instruction_w1_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_instruction_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcastss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcastsd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pbroadcastq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_pi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_destination_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_reg_avx_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_reg_avx_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_reg_general_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_reg_general_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_broadcast_reg_general_reg_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_extract_32x8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_extract_64x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_extract_32x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extractf128_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extractf_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extractf_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extractf_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_insert_64x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_insert_32x8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_insert_64x2_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_insert_32x4_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_insertf128_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_insertf_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_insertf_reg_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extract_b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extract_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extract_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extract_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extract_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extractps_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_extractps_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_insertps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pinsrb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pinsrw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pinsrd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pinsrq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pinsr_instruction_3a( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pinsr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtdq2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2qq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttps2qq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_d_reg_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_d_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtpd2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtuqq2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtpd2udq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttpd2udq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtpd2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttpd2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_size_not_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttps2udq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttps2dq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtph2ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtps2ph_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vcvtps2ph_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vcvtps2ph_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttsd2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsd2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttsd2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtss2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttss2usi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtss2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvttss2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_2si_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_2si_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtusi2sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi2sd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtusi2ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi2ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi_rounding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi_reg_reg_reg32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvtsi_reg_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_maskmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_maskmov_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_movmskpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_movmskps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_movmskps_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_maskmovdqu_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovmskb_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_pmovmskb_reg_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_mem_size_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_elements_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_reg_mem_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_arguments_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_regular( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_double( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_uniform( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gather_vr128( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn scatter_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn scatter_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_qpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_dpd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_pd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_qps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_dps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_ps_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn gatherpf_check_vsib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bmi_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bmi_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn operand_32or64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn operand_32or64_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pdep_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pext_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn andn_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn sarx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn shrx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn shlx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bzhi_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bzhi_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_vex_source_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_vex_source_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bextr_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bextr_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn setup_bextr_imm_opcode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bextr_reg_mem_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bextr_reg_reg_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_nomem_instruction_with_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn rorx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn rorx_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn lwpins_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn lwpins_reg_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn lwpins_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_lwpins( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_single_source_ss_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_single_source_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_instruction_9( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_single_source_128bit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_triple_source_128bit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_128bit_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_instruction_8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_b_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_d_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_q_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_w_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_ub_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_ud_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_uq_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_uw_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_pcom_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vpcmov_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_shift_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_shift_reg_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_shift_reg_reg_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_shift_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_shift_reg_mem_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn reg4_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn take_tile_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn amx_int8_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn amx_bf16_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn amx_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn tilezero_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn tilerelease_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn tile_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn ldtilecfg_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn tileloadd_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn tileloadd_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn tilestored_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_avx_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_register_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn avx_register_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_vex_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_vex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_vex_lpp_bits( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_vex_pp_bits( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vex_f2( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vex_f3( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vex_66( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_vex_0f38_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_vex_0f3a_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_vex_0f_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_c4_vex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_vex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn vex_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_xop_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn xop_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_evex_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_evex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn evex_rounding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn evex_l_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn evex_zaaa_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn evex_b_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_evex_0f38_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_evex_0f3a_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn compress_displacement( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_displacement_scale( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn displacement_not_compressed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn displacement_compressed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn displacement_compression_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn duplicate_output_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extension_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn sys_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn efi_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bin_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn obj_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn o_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn exe_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn adapt_case( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn adapt_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn adapt_next( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extension_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_extension( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extension_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn output_path_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_labels( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn labels_table_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn common_formatter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_code_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stub_written( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_output( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn output_written( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn select_format( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_defined( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn entry_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stack_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn heap_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn segment_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_allowed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extrn_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extrn_allowed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_extrn_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extrn_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn relocation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_pass( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_mz_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_relocation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn segment_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn initial_cs_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_invalid_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn ignore_invalid_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_stack( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stack_pointer( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn initial_ss_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_heap( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn write_mz_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_stack_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mz_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_stub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_stub_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stub_from_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stub_header_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn read_stub_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stub_code_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn stub_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn binary_stub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn binary_heap_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_settings( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn dll_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn wdm_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn large_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn nx_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn subsystem_setting( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn subsystem_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn version_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_version( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn subsystem_version_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_pe_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_peplus_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_base_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_stub_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_settings_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_pe_stub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_stub_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_pe_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_alignment_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn init_peplus_specific( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_header_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_space_for_section_headers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_entry_init_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn peplus_org( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_org_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe64_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_code_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_labels_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn dll_flag_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn wdm_flag_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn large_flag_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn nx_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_section_code_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn peplus_section_org( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_section_org_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_section_flags( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn set_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn peplus_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_directory_set( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_pe_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn align_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_code_sum_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_data_sum_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn udata_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn predefined_data_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn data_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn peplus_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn init_pe_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn end_peplus_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_pe_entry_label_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_entry_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe64_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_pe64_entry_label_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe64_entry_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe64_entry_range_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_stack( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_stack_commit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn peplus_stack( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_peplus_stack_commit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_heap( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn peplus_heap( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_pe_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_standard_pe_relocation_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_relocation_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn fixup_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn fixup_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn generate_pe_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_pe_fixups( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn fixups_ready( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_forward_fixups( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_fixups( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_fixup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn fixups_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_fixup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn fixups_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_pe_resource( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn reserve_space_for_resource( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_from_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn read_resource_headers( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_file_alignment_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_resource_header_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_header_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_resource_header_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_header_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_headers_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_type_name_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_type_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_with_previous_type_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_this_type_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_with_current_type_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn type_name_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn same_type_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_next_type_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn type_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_type_name_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn type_name_directory_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_type_id_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_type_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_next_type_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn type_id_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_type_id_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn type_id_directory_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_resource_directories( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_resource_name_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_with_previous_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_unicode_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_this_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn compare_with_current_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_name_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn same_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_next_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_resource_name_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_name_directory_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_resource_id_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_resource_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_next_resource_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_id_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_resource_id_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_id_directory_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_directories_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn update_resource_directories( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_language_directories( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_language_id_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_language_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn get_language_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_next_language_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn language_id_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_language_id_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn language_id_directory_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn language_directories_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_directories_updated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_name_strings( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_string_entries( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn copy_string_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_data_copied( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn string_entries_processed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_strings_alignment_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn update_language_directories( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_data_records( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn update_data_records( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_data_alignment_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn resource_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_pe( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_sections_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn process_directories( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn directory_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_pe_relocations( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn pe_relocations_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_checksum( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn flat_section_flags_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_labels_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_section_flags( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_section_flag_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_section_alignment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_section_settings_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_coff_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_coff_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_64bit_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_relocation_relative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn relative_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn relative_coff_64bit_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_coff( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_closed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_formatter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_magic_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn coff_flags_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn enumerate_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn enumerate_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn enumerate_public( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn enumerate_extrn( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_default_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_references_to_default_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_reference( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_public_reference( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_other_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbols_enumerated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_name_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_ptr_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_relocations( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn next_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_relocations_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_relocations_count_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_relocations_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn sections_finished( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn zero_symbols_table( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_symbols_table( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_section_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_extrn_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_public_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_coff_public( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn check_64bit_public_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_symbol_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_symbol_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_symbol_class( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn alias_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn symbols_table_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_symbol_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn add_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_header_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_elf64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_labels_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_section_flags( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_section_alignment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_section_settings_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_elf_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_gotoff_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_relocation_relative( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_closed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_formatter( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn align_elf_structures( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_first_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_other_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn first_section_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_next_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_extrn( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_public( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_section_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_section_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_index_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_section_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn find_other_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn skip_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_public_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_public( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_public( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_absolute( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_for_public_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_public_function( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_elf_public_info( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_public_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_public_function( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_elf64_public_info( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn public_symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_extrn_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_extrn_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn extrn_symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_symbol_table_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_string_table( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn rel_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_elf_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_elf64_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_rel_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn default_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_string_table_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_elf64_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_header_finished( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_null_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_section_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn bss_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn section_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn convert_relocations( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_relocation_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn make_elf64_relocation_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn addend_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn relocation_entry_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_elf_machine_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_machine_word_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn relocations_converted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_relocations_name_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn rela_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn store_relocations_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_elf64_rela_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn rel_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn adjust_elf_section_headers_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_elf64_sym_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn sym_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_exe_brand_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_exe_base_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn init_elf_segments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_exe_addressing_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn format_elf64_exe( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_exe_brand_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_exe_base_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn init_elf64_segments( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn setup_elf_exe_labels_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_exe_labels_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_elf_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_flags( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_elf_segment_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_elf_segment_types( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_flags_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_elf_segment_merging( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_addressing_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn merge_elf_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_separated_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn merge_elf_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn first_elf_segment_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_elf_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_position_move( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_position_move_and_align( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_position_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn new_elf64_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_flags( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn mark_elf64_segment_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn scan_elf64_segment_types( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_type_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_flags_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn no_elf64_segment_merging( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn merge_elf64_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_separated_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn merge_elf64_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf64_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn first_elf64_segment_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn finish_elf64_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_position_move( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_position_move_and_align( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_position_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
            return ( rcx, rdx, r8, r9 ) 
        }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf_exe( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
        {
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
23518 */
