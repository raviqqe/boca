require 'aruba/cucumber'

# Cucumber substitutes example values into scenario outlines as replacement templates, so a
# backslash followed by a backtick in a value expands into the text before a match. Collapse
# escaped backslashes, which feature files depend on, and substitute values literally instead.
module Gherkin
  module Pickles
    class Compiler
      def interpolate(name, variable_cells, value_cells)
        variable_cells.each_with_index do |variable_cell, index|
          value = value_cells[index].value.gsub('\\\\') { '\\' }

          name = name.gsub("<#{variable_cell.value}>") { value }
        end

        name
      end
    end
  end
end
